use core::marker::PhantomData;

use cortex_m::asm;
use max32625::FLC;

const START_PTR: *const u32 = 0x3_E000 as *const () as _;
const PAGE_SIZE: u32 = 8 * 1024; // 8KB
const DATA_SIZE: u32 = 16; // 16 Bytes == 4 u32's

#[derive(Debug, PartialEq, PartialOrd)]
pub enum FlashError {
    FirstWordMustHaveZeros,
    FlashError,
}

pub struct Flash<T: FlashData> {
    flc: FLC,
    pointer: *const u32,
    phantom: PhantomData<T>,
}

impl<T: FlashData> Flash<T> {
    pub fn init(flc: FLC) -> Result<Self, FlashError> {
        // Wait for not busy
        while Self::is_busy(&flc) {
            asm::nop()
        }
        // Set up flash
        flc.perform
            .modify(|_, w| w.auto_clkdiv().set_bit().en_prevent_fail().set_bit());
        // Enable fail interrupt
        flc.intr.modify(|_, w| {
            w.failed_ie()
                .set_bit()
                .finished_if()
                .clear_bit()
                .failed_if()
                .clear_bit()
        });
        // Read through data, seeing if there are any settings already saved
        let pointer = match Self::find_start() {
            Some(ptr) => ptr,
            None => {
                // Page erase
                Self::page_erase(&flc, START_PTR)?;
                // Write Default data since there is none
                let data = Self::try_serialize(&T::default())?;
                Self::write_data(&flc, START_PTR, data)?;
                // Return pointer to default
                START_PTR
            }
        };
        // Return
        Ok(Self {
            flc,
            pointer,
            phantom: PhantomData,
        })
    }

    pub fn write_new_data(&mut self, data: &T) -> Result<(), FlashError> {
        // Get new pointer
        let mut ptr = unsafe {
            self.pointer
                .add(DATA_SIZE as usize / core::mem::size_of::<u32>())
        };
        // 0x3_FFFF is the max address of this chip
        if ptr as u32 >= 0x3_FFFF {
            // Page erase to free up more space
            Self::page_erase(&self.flc, START_PTR)?;
            ptr = START_PTR;
        }
        // Write data
        Self::write_data(&self.flc, ptr, Self::try_serialize(data)?)?;
        // Update inner pointer
        self.pointer = ptr;
        Ok(())
    }

    pub fn get_data(&self) -> Result<T, FlashError> {
        if self.pointer as u32 >= START_PTR as u32
            && self.pointer as u32 <= (START_PTR as u32 + PAGE_SIZE - DATA_SIZE)
        {
            let data = unsafe { &*(self.pointer as *const [u32; 4]) };
            Ok(T::from_bytes(data))
        } else {
            Err(FlashError::FlashError)
        }
    }

    fn is_busy(flc: &FLC) -> bool {
        let reg = flc.ctrl.read();
        reg.write().bit_is_set() | reg.page_erase().bit_is_set() | reg.mass_erase().bit_is_set()
    }

    fn try_serialize(data: &T) -> Result<[u32; 4], FlashError> {
        let data = data.serialize();
        if data[0] == 0xFFFF_FFFF {
            Err(FlashError::FirstWordMustHaveZeros)
        } else {
            Ok(data)
        }
    }

    fn find_start() -> Option<*const u32> {
        // Read until we find an invalid block
        // If we find nothing, return none
        let mut ret = None;
        let mut ptr = START_PTR;
        while unsafe { *ptr != 0xFFFF_FFFF } {
            ret = Some(ptr);
            ptr = unsafe { ptr.add(DATA_SIZE as usize / core::mem::size_of::<u32>()) };
            // 0x3_FFFF is the max address of this chip
            if ptr as u32 >= 0x3_FFFF {
                break;
            }
        }
        ret
    }

    fn write_data(flc: &FLC, addr: *const u32, data: [u32; 4]) -> Result<(), FlashError> {
        /* Check if the flash controller is busy */
        while Self::is_busy(flc) {
            asm::nop()
        }

        /* Clear stale errors. */
        flc.intr
            .modify(|_, w| w.failed_if().clear_bit().finished_if().clear_bit());

        /* Unlock flash */
        flc.ctrl.modify(|_, w| w.flsh_unlock().variant(2));

        /* Set the address to write and enable auto increment */
        flc.faddr.write(|w| w.faddr().variant(addr as u32));
        flc.ctrl.modify(|_, w| w.auto_incre_mode().set_bit());

        for word in data {
            /* Perform the write */
            flc.fdata.write(|w| unsafe { w.bits(word) });
            flc.ctrl.modify(|_, w| w.write().set_bit());
            while Self::is_busy(flc) {
                asm::nop()
            }
        }

        /* Lock flash */
        flc.ctrl.modify(|_, w| w.flsh_unlock().variant(0));

        /* Check for failures */
        if flc.intr.read().failed_if().bit_is_set() {
            /* Interrupt flags can only be written to zero, so this is safe */
            flc.intr.modify(|_, w| w.failed_if().clear_bit());
            return Err(FlashError::FlashError);
        }
        Ok(())
    }

    fn page_erase(flc: &FLC, addr: *const u32) -> Result<(), FlashError> {
        /* Check if the flash controller is busy */
        while Self::is_busy(flc) {
            asm::nop()
        }

        /* Clear stale errors */
        flc.intr
            .modify(|_, w| w.failed_if().clear_bit().finished_if().clear_bit());

        /* Unlock flash */
        flc.ctrl.modify(|_, w| w.flsh_unlock().variant(2));

        /* Write the Erase Code */
        flc.ctrl.modify(|_, w| w.erase_code().variant(0x55));

        /* Erase the request page */
        flc.faddr.write(|w| w.faddr().variant(addr as u32));
        flc.ctrl.modify(|_, w| w.page_erase().set_bit());

        /* Wait until flash operation is complete */
        while Self::is_busy(flc) {
            asm::nop()
        }

        /* Lock flash */
        flc.ctrl
            .modify(|_, w| w.flsh_unlock().variant(0).erase_code().variant(0));

        /* Check for failures */
        if flc.intr.read().failed_if().bit_is_set() {
            /* Interrupt flags can only be written to zero, so this is safe */
            flc.intr.modify(|_, w| w.failed_if().clear_bit());
            return Err(FlashError::FlashError);
        }
        Ok(())
    }
}

unsafe impl<T: FlashData> Send for Flash<T> {}

/// Rules:
/// 1. Must be valid given any state,
/// 2. the first u32 in the serialized version must never be all ones,
/// 3. and it must be less than 16 bytes in size.
pub trait FlashData: Default {
    fn serialize(&self) -> [u32; 4];
    fn from_bytes(bytes: &[u32; 4]) -> Self;
}
