use usbd_hid::descriptor::generator_prelude::*;

pub enum ReportTypes {
    MouseReport,
    CommandResponse,
    LiftReport,
    CpiReport,
    SensorImage,
}

pub enum Reports {
    Mouse(MouseReport),
    KeyboardCpi(KeyboardReport),
    KeyboardLift(KeyboardReport),
    Command(CommandReport),
    SensorImage(SensorReport),
}

#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = MOUSE) = {
        (collection = PHYSICAL, usage = POINTER) = {
            (usage_page = BUTTON, usage_min = BUTTON_1, usage_max = BUTTON_8) = {
                #[packed_bits 8] #[item_settings data,variable,absolute] buttons=input;
            };
            (usage_page = GENERIC_DESKTOP,) = {
                (usage = X,) = {
                    #[item_settings data,variable,relative] x=input;
                };
                (usage = Y,) = {
                    #[item_settings data,variable,relative] y=input;
                };
                (usage = WHEEL,) = {
                    #[item_settings data,variable,relative] wheel=input;
                };
            };
        };
    }
)]
pub struct MouseReport {
    pub buttons: u8,
    pub x: i16,
    pub y: i16,
    pub wheel: i8,
}

#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = KEYBOARD) = {
        (usage_page = KEYBOARD, usage_min = 0xE0, usage_max = 0xE7) = {
            #[packed_bits 8] #[item_settings data,variable,absolute] modifier=input;
        };
        (usage_page = KEYBOARD, usage_min = 0x00, usage_max = 0xDD) = {
            #[item_settings data,array,absolute] keycodes=input;
        };
    }
)]
#[allow(dead_code)]
pub struct KeyboardReport {
    pub modifier: u8,
    pub keycodes: [u8; 6],
}

#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = VENDOR_DEFINED_START, usage = 0x01) = {
        (usage = 0x01, usage_min = 0x00, usage_max = 0xFF) = {
            #[item_settings data,variable] command=feature;
        };
        (usage = 0x01, usage_min = 0x00, usage_max = 0xFF) = {
            #[item_settings data,array] args=feature;
        };
        (usage = 0x01, usage_min = 0x00, usage_max = 0xFF) = {
            #[item_settings data,variable] response=input;
        };
        (usage = 0x01, usage_min = 0x00, usage_max = 0xFF) = {
            #[item_settings data,array] data=input;
        };
    }
)]
pub struct CommandReport {
    pub command: u8,
    pub args: [u8; 4],
    pub response: u8,
    pub data: [u8; 4],
}

#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = VENDOR_DEFINED_START, usage = 0x02) = {
        (usage = 0x02, usage_min = 0x00, usage_max = 0xFF) = {
            #[item_settings data,variable] large_data_ind=input;
        };
        (usage = 0x02, usage_min = 0x00, usage_max = 0xFF) = {
            #[item_settings data,array] large_data=input;
        };
    }
)]
pub struct SensorReport {
    pub large_data_ind: u8,
    pub large_data: [u8; 62],
}
