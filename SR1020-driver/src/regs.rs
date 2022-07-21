const fn byte_mask(max_ind: u8, min_ind: u8) -> u8 {
    (0xff >> (7 - (max_ind))) & !((1 << (min_ind)) - 1)
}

mod reg_structs {
    use super::byte_mask;
    pub struct REG_STATUS1 {
        pub(crate) value: u8,
    }
    impl REG_STATUS1 {
        pub fn STAT2IRQ(&self) -> bool {
            self.value & (1 << 7) != 0
        }
        pub fn PKBEGINI(&self) -> bool {
            self.value & (1 << 6) != 0
        }
        pub fn RXTIMEOI(&self) -> bool {
            self.value & (1 << 5) != 0
        }
        pub fn TXENDI(&self) -> bool {
            self.value & (1 << 4) != 0
        }
        pub fn NEWPKTI(&self) -> bool {
            self.value & (1 << 3) != 0
        }
        pub fn ADDRMATI(&self) -> bool {
            self.value & (1 << 2) != 0
        }
        pub fn BRDCASTI(&self) -> bool {
            self.value & (1 << 1) != 0
        }
        pub fn CRCPASSI(&self) -> bool {
            self.value & (1 << 0) != 0
        }
    }
    pub struct REG_IRQMASK1 {
        pub(crate) value: u8,
    }
    impl REG_IRQMASK1 {
        pub fn IRQPOLAR(&self) -> bool {
            self.value & (1 << 7) != 0
        }
        pub fn PKBEGINE(&self) -> bool {
            self.value & (1 << 6) != 0
        }
        pub fn RXTIMEOE(&self) -> bool {
            self.value & (1 << 5) != 0
        }
        pub fn TXENDE(&self) -> bool {
            self.value & (1 << 4) != 0
        }
        pub fn NEWPKTE(&self) -> bool {
            self.value & (1 << 3) != 0
        }
        pub fn ADDRMATE(&self) -> bool {
            self.value & (1 << 2) != 0
        }
        pub fn BRDCASTE(&self) -> bool {
            self.value & (1 << 1) != 0
        }
        pub fn CRCPASSE(&self) -> bool {
            self.value & (1 << 0) != 0
        }
    }
    pub struct REG_STATUS2 {
        pub(crate) value: u8,
    }
    impl REG_STATUS2 {
        pub fn XOTIMERI(&self) -> bool {
            self.value & (1 << 7) != 0
        }
        pub fn WAKEUPI(&self) -> bool {
            self.value & (1 << 6) != 0
        }
        pub fn CSCFAILI(&self) -> bool {
            self.value & (1 << 5) != 0
        }
        pub fn TXUDRFLI(&self) -> bool {
            self.value & (1 << 4) != 0
        }
        pub fn RXOVRFLI(&self) -> bool {
            self.value & (1 << 3) != 0
        }
        pub fn TXOVRFLI(&self) -> bool {
            self.value & (1 << 2) != 0
        }
        pub fn BUFLOADI(&self) -> bool {
            self.value & (1 << 1) != 0
        }
        pub fn BUFSTOPI(&self) -> bool {
            self.value & (1 << 0) != 0
        }
    }
    pub struct REG_IRQMASK2 {
        pub(crate) value: u8,
    }
    impl REG_IRQMASK2 {
        pub fn XOTIMERE(&self) -> bool {
            self.value & (1 << 7) != 0
        }
        pub fn WAKEUPE(&self) -> bool {
            self.value & (1 << 6) != 0
        }
        pub fn CSCFAILE(&self) -> bool {
            self.value & (1 << 5) != 0
        }
        pub fn TXUDRFLE(&self) -> bool {
            self.value & (1 << 4) != 0
        }
        pub fn RXOVRFLE(&self) -> bool {
            self.value & (1 << 3) != 0
        }
        pub fn TXOVRFLE(&self) -> bool {
            self.value & (1 << 2) != 0
        }
        pub fn BUFLOADE(&self) -> bool {
            self.value & (1 << 1) != 0
        }
        pub fn BUFSTOPE(&self) -> bool {
            self.value & (1 << 0) != 0
        }
    }
    pub struct REG_TXFIFOSTAT {
        pub(crate) value: u8,
    }
    impl REG_TXFIFOSTAT {
        pub fn TXBUFLOAD(&self) -> u8 {
            (self.value & byte_mask(7, 0)) >> 0
        }
    }
    pub struct REG_TXFIFOTHRESH {
        pub(crate) value: u8,
    }
    impl REG_TXFIFOTHRESH {
        pub fn TXIRQEN(&self) -> bool {
            self.value & (1 << 7) != 0
        }
        pub fn TXTHRESH(&self) -> u8 {
            (self.value & byte_mask(6, 0)) >> 0
        }
    }
    pub struct REG_RXFIFOSTAT {
        pub(crate) value: u8,
    }
    impl REG_RXFIFOSTAT {
        pub fn RXBUFLOAD(&self) -> u8 {
            (self.value & byte_mask(7, 0)) >> 0
        }
    }
    pub struct REG_RXFIFOTHRESH {
        pub(crate) value: u8,
    }
    impl REG_RXFIFOTHRESH {
        pub fn RXIRQEN(&self) -> bool {
            self.value & (1 << 7) != 0
        }
        pub fn RXTHRESH(&self) -> u8 {
            (self.value & byte_mask(6, 0)) >> 0
        }
    }
    pub struct REG_SLEEPCONF {
        pub(crate) value: u8,
    }
    impl REG_SLEEPCONF {
        pub fn SLPDEPTH(&self) -> u8 {
            (self.value & byte_mask(7, 6)) >> 6
        }
        pub fn SLPRXTO(&self) -> bool {
            self.value & (1 << 5) != 0
        }
        pub fn SLPTXEND(&self) -> bool {
            self.value & (1 << 4) != 0
        }
        pub fn SLPRXEND(&self) -> bool {
            self.value & (1 << 3) != 0
        }
        pub fn SLPMATCH(&self) -> bool {
            self.value & (1 << 2) != 0
        }
        pub fn SLPBRDCA(&self) -> bool {
            self.value & (1 << 1) != 0
        }
        pub fn SLPNOISY(&self) -> bool {
            self.value & (1 << 0) != 0
        }
    }
    pub struct REG_TIMERCONF {
        pub(crate) value: u8,
    }
    impl REG_TIMERCONF {
        pub fn AUTOWAKE(&self) -> bool {
            self.value & (1 << 7) != 0
        }
        pub fn WAKEONCE(&self) -> bool {
            self.value & (1 << 6) != 0
        }
        pub fn SYNATEND(&self) -> bool {
            self.value & (1 << 5) != 0
        }
        pub fn SYNTXSTA(&self) -> bool {
            self.value & (1 << 4) != 0
        }
        pub fn SYNRXSTA(&self) -> bool {
            self.value & (1 << 3) != 0
        }
        pub fn SYNMATCH(&self) -> bool {
            self.value & (1 << 2) != 0
        }
        pub fn SYNBRDCA(&self) -> bool {
            self.value & (1 << 1) != 0
        }
        pub fn SYNRXCRC(&self) -> bool {
            self.value & (1 << 0) != 0
        }
    }
    pub struct REG_TIMERCOUNT1 {
        pub(crate) value: u8,
    }
    impl REG_TIMERCOUNT1 {
        pub fn XTALCOUNT8(&self) -> u8 {
            (self.value & byte_mask(7, 0)) >> 0
        }
    }
    pub struct REG_TIMERPERIOD1 {
        pub(crate) value: u8,
    }
    impl REG_TIMERPERIOD1 {
        pub fn SLPPERIOD8(&self) -> u8 {
            (self.value & byte_mask(7, 0)) >> 0
        }
    }
    pub struct REG_TIMERCOUNT0 {
        pub(crate) value: u8,
    }
    impl REG_TIMERCOUNT0 {
        pub fn XTALCOUNT0(&self) -> u8 {
            (self.value & byte_mask(7, 0)) >> 0
        }
    }
    pub struct REG_TIMERPERIOD0 {
        pub(crate) value: u8,
    }
    impl REG_TIMERPERIOD0 {
        pub fn SLPPERIOD0(&self) -> u8 {
            (self.value & byte_mask(7, 0)) >> 0
        }
    }
    pub struct REG_RXTIMEOUT1 {
        pub(crate) value: u8,
    }
    impl REG_RXTIMEOUT1 {
        pub fn RXPERIOD4(&self) -> u8 {
            (self.value & byte_mask(7, 0)) >> 0
        }
    }
    pub struct REG_RXTIMEOUT0 {
        pub(crate) value: u8,
    }
    impl REG_RXTIMEOUT0 {
        pub fn RXPERIOD0(&self) -> u8 {
            (self.value & byte_mask(7, 4)) >> 4
        }
        pub fn RXPUDELAY(&self) -> u8 {
            (self.value & byte_mask(3, 0)) >> 0
        }
    }
    pub struct REG_PWRUPDELAY {
        pub(crate) value: u8,
    }
    impl REG_PWRUPDELAY {
        pub fn PWRUPDEL(&self) -> u8 {
            (self.value & byte_mask(7, 0)) >> 0
        }
    }
    pub struct REG_PLLSTARTUP {
        pub(crate) value: u8,
    }
    impl REG_PLLSTARTUP {
        pub fn PLLPUWAIT(&self) -> u8 {
            (self.value & byte_mask(7, 0)) >> 0
        }
    }
    pub struct REG_PLLRATIO {
        pub(crate) value: u8,
    }
    impl REG_PLLRATIO {
        pub fn RPLYTXDEL(&self) -> u8 {
            (self.value & byte_mask(7, 6)) >> 6
        }
        pub fn PLLRATIO(&self) -> u8 {
            (self.value & byte_mask(5, 0)) >> 0
        }
    }
    pub struct REG_RESISTUNE {
        pub(crate) value: u8,
    }
    impl REG_RESISTUNE {
        pub fn LNAIMPED(&self) -> bool {
            self.value & (1 << 7) != 0
        }
        pub fn PLLRES(&self) -> u8 {
            (self.value & byte_mask(6, 4)) >> 4
        }
        pub fn VREFTUNE(&self) -> u8 {
            (self.value & byte_mask(3, 0)) >> 0
        }
    }
    pub struct REG_DISABLES {
        pub(crate) value: u8,
    }
    impl REG_DISABLES {
        pub fn STDSPI(&self) -> bool {
            self.value & (1 << 7) != 0
        }
        pub fn AUTOFLUSHDIS(&self) -> bool {
            self.value & (1 << 6) != 0
        }
        pub fn ONEVSWDIS(&self) -> bool {
            self.value & (1 << 5) != 0
        }
        pub fn DCDCDIS(&self) -> bool {
            self.value & (1 << 4) != 0
        }
        pub fn PLLDIS(&self) -> bool {
            self.value & (1 << 3) != 0
        }
        pub fn SYMBCSRC(&self) -> bool {
            self.value & (1 << 2) != 0
        }
        pub fn XTALCSRC(&self) -> bool {
            self.value & (1 << 1) != 0
        }
        pub fn OUTPXTAL(&self) -> bool {
            self.value & (1 << 0) != 0
        }
    }
    pub struct REG_RXFILTERS {
        pub(crate) value: u8,
    }
    impl REG_RXFILTERS {
        pub fn LNAPEAK(&self) -> u8 {
            (self.value & byte_mask(7, 5)) >> 5
        }
        pub fn RFFILFREQ(&self) -> u8 {
            (self.value & byte_mask(4, 0)) >> 0
        }
    }
    pub struct REG_TXPULSE12 {
        pub(crate) value: u8,
    }
    impl REG_TXPULSE12 {}
    pub struct REG_TXPULSE11 {
        pub(crate) value: u8,
    }
    impl REG_TXPULSE11 {}
    pub struct REG_TXPULSE10 {
        pub(crate) value: u8,
    }
    impl REG_TXPULSE10 {}
    pub struct REG_TXPULSE9 {
        pub(crate) value: u8,
    }
    impl REG_TXPULSE9 {}
    pub struct REG_TXPULSE8 {
        pub(crate) value: u8,
    }
    impl REG_TXPULSE8 {}
    pub struct REG_TXPULSE7 {
        pub(crate) value: u8,
    }
    impl REG_TXPULSE7 {}
    pub struct REG_TXPULSE6 {
        pub(crate) value: u8,
    }
    impl REG_TXPULSE6 {}
    pub struct REG_TXPULSE5 {
        pub(crate) value: u8,
    }
    impl REG_TXPULSE5 {}
    pub struct REG_TXPULSE4 {
        pub(crate) value: u8,
    }
    impl REG_TXPULSE4 {}
    pub struct REG_TXPULSE3 {
        pub(crate) value: u8,
    }
    impl REG_TXPULSE3 {}
    pub struct REG_TXPULSE2 {
        pub(crate) value: u8,
    }
    impl REG_TXPULSE2 {}
    pub struct REG_TXPULSE1 {
        pub(crate) value: u8,
    }
    impl REG_TXPULSE1 {
        pub fn PULSEWID(&self) -> u8 {
            (self.value & byte_mask(7, 5)) >> 5
        }
        pub fn PULSEFREQ(&self) -> u8 {
            (self.value & byte_mask(4, 0)) >> 0
        }
    }
    pub struct REG_TXPARAMS {
        pub(crate) value: u8,
    }
    impl REG_TXPARAMS {
        pub fn HOLDTXON(&self) -> bool {
            self.value & (1 << 7) != 0
        } /* RW */
        pub fn RNDPHASE(&self) -> bool {
            self.value & (1 << 6) != 0
        } /* RW */
        pub fn TXPOWER(&self) -> u8 {
            (self.value & byte_mask(5, 4)) >> 4
        } /* RW */
        pub fn IFFILTEN(&self) -> bool {
            self.value & (1 << 3) != 0
        } /* RW */
        pub fn VDDLEVEL(&self) -> u8 {
            (self.value & byte_mask(2, 0)) >> 0
        } /* RO */
    }
    pub struct REG_DLLTUNING {
        pub(crate) value: u8,
    }
    impl REG_DLLTUNING {
        pub fn DLTUNE(&self) -> u8 {
            (self.value & byte_mask(7, 4)) >> 4
        } /* RW */
        pub fn ECO(&self) -> bool {
            self.value & (1 << 3) != 0
        } /* WO */
        pub fn LEADLAG(&self) -> bool {
            self.value & (1 << 3) != 0
        } /* RO */
        pub fn INTEGLEN(&self) -> bool {
            self.value & (1 << 2) != 0
        } /* RW */
        pub fn INTEGGAIN(&self) -> u8 {
            (self.value & byte_mask(1, 0)) >> 0
        } /* RW */
    }
    pub struct REG_CALIBREQUEST {
        pub(crate) value: u8,
    }
    impl REG_CALIBREQUEST {
        pub fn DCROCODE(&self) -> u8 {
            (self.value & byte_mask(4, 0)) >> 0
        }
    }
    pub struct REG_CALIBRESULT {
        pub(crate) value: u8,
    }
    impl REG_CALIBRESULT {
        pub fn DCROFREQ(&self) -> u8 {
            (self.value & byte_mask(7, 0)) >> 0
        }
    }
    pub struct REG_PWRSTATUS {
        pub(crate) value: u8,
    }
    impl REG_PWRSTATUS {
        pub fn ROMEN(&self) -> bool {
            self.value & (1 << 7) != 0
        }
        pub fn RXEN(&self) -> bool {
            self.value & (1 << 6) != 0
        }
        pub fn TXEN(&self) -> bool {
            self.value & (1 << 5) != 0
        }
        pub fn AWAKE(&self) -> bool {
            self.value & (1 << 4) != 0
        }
        pub fn MODEMON(&self) -> bool {
            self.value & (1 << 3) != 0
        }
        pub fn DCDCEN(&self) -> bool {
            self.value & (1 << 2) != 0
        }
        pub fn PLLEN(&self) -> bool {
            self.value & (1 << 1) != 0
        }
        pub fn REFEN(&self) -> bool {
            self.value & (1 << 0) != 0
        }
    }
    pub struct REG_ACTIONS {
        pub(crate) value: u8,
    }
    impl REG_ACTIONS {
        pub fn CALIBRAT(&self) -> bool {
            self.value & (1 << 7) != 0
        }
        pub fn SKIPWAKE(&self) -> bool {
            self.value & (1 << 6) != 0
        }
        pub fn RXMODE(&self) -> bool {
            self.value & (1 << 5) != 0
        }
        pub fn STARTTX(&self) -> bool {
            self.value & (1 << 4) != 0
        }
        pub fn INITTIME(&self) -> bool {
            self.value & (1 << 3) != 0
        }
        pub fn GOTOSLP(&self) -> bool {
            self.value & (1 << 2) != 0
        }
        pub fn FLUSHRX(&self) -> bool {
            self.value & (1 << 1) != 0
        }
        pub fn FLUSHTX(&self) -> bool {
            self.value & (1 << 0) != 0
        }
    }
    pub struct REG_NVMVALUE {
        pub(crate) value: u8,
    }
    impl REG_NVMVALUE {
        pub fn ROMBYTE(&self) -> u8 {
            (self.value & byte_mask(7, 0)) >> 0
        }
    }
    pub struct REG_NVMADDRESS {
        pub(crate) value: u8,
    }
    impl REG_NVMADDRESS {
        pub fn ROMPWRSW(&self) -> bool {
            self.value & (1 << 7) != 0
        }
        pub fn ROMADDR(&self) -> u8 {
            (self.value & byte_mask(6, 0)) >> 0
        }
    }
    pub struct REG_BITTHRES {
        pub(crate) value: u8,
    }
    impl REG_BITTHRES {
        pub fn BITTHRES(&self) -> u8 {
            (self.value & byte_mask(3, 0)) >> 0
        }
    }
    pub struct REG_RSSI {
        pub(crate) value: u8,
    }
    impl REG_RSSI {
        pub fn RSSI(&self) -> u8 {
            (self.value & byte_mask(5, 0)) >> 0
        }
    }
    pub struct REG_RNSI {
        pub(crate) value: u8,
    }
    impl REG_RNSI {
        pub fn RNSI(&self) -> u8 {
            (self.value & byte_mask(5, 0)) >> 0
        }
    }
    pub struct REG_RXWAITTIME1 {
        pub(crate) value: u8,
    }
    impl REG_RXWAITTIME1 {
        pub fn RXWAISRC(&self) -> bool {
            self.value & (1 << 7) != 0
        } /* RW */
        pub fn RXWAITED8(&self) -> u8 {
            (self.value & byte_mask(6, 0)) >> 0
        } /* RO */
    }
    pub struct REG_RXWAITTIME0 {
        pub(crate) value: u8,
    }
    impl REG_RXWAITTIME0 {
        pub fn RXWAITED0(&self) -> u8 {
            (self.value & byte_mask(7, 0)) >> 0
        }
    }
    pub struct REG_FRAMEADDR1 {
        pub(crate) value: u8,
    }
    impl REG_FRAMEADDR1 {
        pub fn PKTADDR8(&self) -> u8 {
            (self.value & byte_mask(7, 0)) >> 0
        }
    }
    pub struct REG_FRAMEADDR0 {
        pub(crate) value: u8,
    }
    impl REG_FRAMEADDR0 {
        pub fn PKTADDR0(&self) -> u8 {
            (self.value & byte_mask(7, 0)) >> 0
        }
    }
    pub struct REG_NOISELVL {
        pub(crate) value: u8,
    }
    impl REG_NOISELVL {
        pub fn MAXADCSIG(&self) -> u8 {
            (self.value & byte_mask(7, 4)) >> 4
        }
        pub fn MINADCSIG(&self) -> u8 {
            (self.value & byte_mask(3, 0)) >> 0
        }
    }
    pub struct REG_PHSDATA1 {
        pub(crate) value: u8,
    }
    impl REG_PHSDATA1 {
        pub fn PHSDATA1(&self) -> u8 {
            (self.value & byte_mask(7, 0)) >> 0
        }
    }
    pub struct REG_PREFRAMECFG {
        pub(crate) value: u8,
    }
    impl REG_PREFRAMECFG {
        pub fn DLAYRPLY(&self) -> bool {
            self.value & (1 << 7) != 0
        }
        pub fn NUMPREDET(&self) -> u8 {
            (self.value & byte_mask(6, 4)) >> 4
        }
        pub fn PREATHRES(&self) -> u8 {
            (self.value & byte_mask(3, 0)) >> 0
        }
    }
    pub struct REG_PHSDATA2 {
        pub(crate) value: u8,
    }
    impl REG_PHSDATA2 {
        pub fn PHSDATA2(&self) -> u8 {
            (self.value & byte_mask(7, 0)) >> 0
        }
    }
    pub struct REG_MODEMGAIN {
        pub(crate) value: u8,
    }
    impl REG_MODEMGAIN {
        pub fn DATASRC(&self) -> u8 {
            (self.value & byte_mask(7, 6)) >> 6
        }
        pub fn MANUGAIN(&self) -> u8 {
            (self.value & byte_mask(5, 0)) >> 0
        }
    }
    pub struct REG_PHSDATA3 {
        pub(crate) value: u8,
    }
    impl REG_PHSDATA3 {
        pub fn PHSDATA3(&self) -> u8 {
            (self.value & byte_mask(7, 0)) >> 0
        }
    }
    pub struct REG_DEBUGMODEM {
        pub(crate) value: u8,
    }
    impl REG_DEBUGMODEM {
        pub fn OVERRIDE(&self) -> bool {
            self.value & (1 << 7) != 0
        }
        pub fn MANUPHASE(&self) -> u8 {
            (self.value & byte_mask(6, 5)) >> 5
        }
        pub fn MANBITHR(&self) -> bool {
            self.value & (1 << 4) != 0
        }
        pub fn BITTHRADJ(&self) -> bool {
            self.value & (1 << 3) != 0
        }
    }
    pub struct REG_PHSDATA4 {
        pub(crate) value: u8,
    }
    impl REG_PHSDATA4 {
        pub fn PHSDATA4(&self) -> u8 {
            (self.value & byte_mask(7, 0)) >> 0
        }
    }
    pub struct REG_MAINMODEM {
        pub(crate) value: u8,
    }
    impl REG_MAINMODEM {
        pub fn AUTOTX(&self) -> bool {
            self.value & (1 << 7) != 0
        }
        pub fn AUTORPLY(&self) -> bool {
            self.value & (1 << 6) != 0
        }
        pub fn ISIMITIG(&self) -> u8 {
            (self.value & byte_mask(5, 4)) >> 4
        }
        pub fn MODULATION(&self) -> u8 {
            (self.value & byte_mask(3, 2)) >> 2
        }
        pub fn FECLEVEL(&self) -> u8 {
            (self.value & byte_mask(1, 0)) >> 0
        }
    }
    pub struct REG_CLEARTOSEND {
        pub(crate) value: u8,
    }
    impl REG_CLEARTOSEND {
        pub fn IDLERXPWR(&self) -> u8 {
            (self.value & byte_mask(7, 6)) >> 6
        }
        pub fn CSTHRES(&self) -> u8 {
            (self.value & byte_mask(5, 0)) >> 0
        }
    }
    pub struct REG_RXPAUSETIME {
        pub(crate) value: u8,
    }
    impl REG_RXPAUSETIME {
        pub fn RXOFFTIME(&self) -> u8 {
            (self.value & byte_mask(7, 0)) >> 0
        }
    }
    pub struct REG_PREAMBLEN {
        pub(crate) value: u8,
    }
    impl REG_PREAMBLEN {
        pub fn PREAMBLEN(&self) -> u8 {
            (self.value & byte_mask(7, 0)) >> 0
        }
    }
    pub struct REG_CONSTGAINS {
        pub(crate) value: u8,
    }
    impl REG_CONSTGAINS {
        pub fn IFGAIN3(&self) -> u8 {
            (self.value & byte_mask(7, 5)) >> 5
        }
        pub fn IFOAGAIN(&self) -> u8 {
            (self.value & byte_mask(4, 2)) >> 2
        }
        pub fn RFGAIN(&self) -> u8 {
            (self.value & byte_mask(1, 0)) >> 0
        }
    }
    pub struct REG_SYNCWORDCFG {
        pub(crate) value: u8,
    }
    impl REG_SYNCWORDCFG {
        pub fn SWLENGTH(&self) -> bool {
            self.value & (1 << 7) != 0
        }
        pub fn SWBITCOST(&self) -> u8 {
            (self.value & byte_mask(6, 5)) >> 5
        }
        pub fn SWCORRTOL(&self) -> u8 {
            (self.value & byte_mask(4, 0)) >> 0
        }
    }
    pub struct REG_SYNCWORD3 {
        pub(crate) value: u8,
    }
    impl REG_SYNCWORD3 {
        pub fn SYNCWORD24(&self) -> u8 {
            (self.value & byte_mask(7, 0)) >> 0
        }
    }
    pub struct REG_SYNCWORD2 {
        pub(crate) value: u8,
    }
    impl REG_SYNCWORD2 {
        pub fn SYNCWORD16(&self) -> u8 {
            (self.value & byte_mask(7, 0)) >> 0
        }
    }
    pub struct REG_SYNCWORD1 {
        pub(crate) value: u8,
    }
    impl REG_SYNCWORD1 {
        pub fn SYNCWORD8(&self) -> u8 {
            (self.value & byte_mask(7, 0)) >> 0
        }
    }
    pub struct REG_SYNCWORD0 {
        pub(crate) value: u8,
    }
    impl REG_SYNCWORD0 {
        pub fn SYNCWORD0(&self) -> u8 {
            (self.value & byte_mask(7, 0)) >> 0
        }
    }
    pub struct REG_CRCPOLYNOM1 {
        pub(crate) value: u8,
    }
    impl REG_CRCPOLYNOM1 {
        pub fn CRCPOLYNO8(&self) -> u8 {
            (self.value & byte_mask(7, 0)) >> 0
        }
    }
    pub struct REG_CRCPOLYNOM0 {
        pub(crate) value: u8,
    }
    impl REG_CRCPOLYNOM0 {
        pub fn CRCPOLYNO0(&self) -> u8 {
            (self.value & byte_mask(7, 0)) >> 0
        }
    }
    pub struct REG_REMOTADDR1 {
        pub(crate) value: u8,
    }
    impl REG_REMOTADDR1 {
        pub fn REMOTADDR8(&self) -> u8 {
            (self.value & byte_mask(7, 0)) >> 0
        }
    }
    pub struct REG_REMOTADDR0 {
        pub(crate) value: u8,
    }
    impl REG_REMOTADDR0 {
        pub fn REMOTADDR0(&self) -> u8 {
            (self.value & byte_mask(7, 0)) >> 0
        }
    }
    pub struct REG_LOCALADDR1 {
        pub(crate) value: u8,
    }
    impl REG_LOCALADDR1 {
        pub fn LOCALADDR8(&self) -> u8 {
            (self.value & byte_mask(7, 0)) >> 0
        }
    }
    pub struct REG_LOCALADDR0 {
        pub(crate) value: u8,
    }
    impl REG_LOCALADDR0 {
        pub fn LOCALADDR0(&self) -> u8 {
            (self.value & byte_mask(7, 0)) >> 0
        }
    }
    pub struct REG_TXPKTSIZE {
        pub(crate) value: u8,
    }
    impl REG_TXPKTSIZE {
        pub fn TXPKTSIZE(&self) -> u8 {
            (self.value & byte_mask(7, 0)) >> 0
        }
    }
    pub struct REG_RXPKTSIZE {
        pub(crate) value: u8,
    }
    impl REG_RXPKTSIZE {
        pub fn RXPKTSIZE(&self) -> u8 {
            (self.value & byte_mask(7, 0)) >> 0
        }
    }
    pub struct REG_PACKETCFG {
        pub(crate) value: u8,
    }
    impl REG_PACKETCFG {
        pub fn ADDRFILT(&self) -> u8 {
            (self.value & byte_mask(7, 6)) >> 6
        }
        pub fn ADDRLEN(&self) -> bool {
            self.value & (1 << 5) != 0
        }
        pub fn ADDRHDRE(&self) -> bool {
            self.value & (1 << 4) != 0
        }
        pub fn SIZEHDRE(&self) -> bool {
            self.value & (1 << 3) != 0
        }
        pub fn SIZESRC(&self) -> bool {
            self.value & (1 << 2) != 0
        }
        pub fn SAVEADDR(&self) -> bool {
            self.value & (1 << 1) != 0
        }
        pub fn SAVESIZE(&self) -> bool {
            self.value & (1 << 0) != 0
        }
    }
    pub struct REG_RXFIFO {
        pub(crate) value: u8,
    }
    impl REG_RXFIFO {
        pub fn RXBUFFER(&self) -> u8 {
            (self.value & byte_mask(7, 0)) >> 0
        }
    }
    pub struct REG_TXFIFO {
        pub(crate) value: u8,
    }
    impl REG_TXFIFO {
        pub fn TXBUFFER(&self) -> u8 {
            (self.value & byte_mask(7, 0)) >> 0
        }
    }
}

mod read_reg {
    use super::reg_structs::*;
    pub struct ReadRegs {}

    macro_rules! read_regs_impl {
        ($struct_name:ident => $($var_name:ident = $var_address:expr => $var_struct_name:ident),+) => {
            impl $struct_name {
                $(
                    pub fn $var_name() -> $var_struct_name {
                        const address: u8 = $var_address;
                        let value = 0x08;
                        $var_struct_name { value }
                    }
                )+
            }
        };
    }

    read_regs_impl! { ReadRegs =>
        STATUS1 = 0x00 => REG_STATUS1,
        STATUS2 = 0x01 => REG_STATUS2,
        TXFIFOSTAT = 0x02 => REG_TXFIFOSTAT,
        RXFIFOSTAT = 0x03 => REG_RXFIFOSTAT,
        SLEEPCONF = 0x04 => REG_SLEEPCONF,
        TIMERCONF = 0x05 => REG_TIMERCONF,
        TIMERCOUNT1 = 0x06 => REG_TIMERCOUNT1,
        TIMERCOUNT0 = 0x07 => REG_TIMERCOUNT0,
        RXTIMEOUT1 = 0x08 => REG_RXTIMEOUT1,
        RXTIMEOUT0 = 0x09 => REG_RXTIMEOUT0,
        PWRUPDELAY = 0x0A => REG_PWRUPDELAY,
        PLLSTARTUP = 0x0B => REG_PLLSTARTUP,
        PLLRATIO = 0x0C => REG_PLLRATIO,
        RESISTUNE = 0x0D => REG_RESISTUNE,
        DISABLES = 0x0E => REG_DISABLES,
        RXFILTERS = 0x0F => REG_RXFILTERS,
        TXPULSE12 = 0x10 => REG_TXPULSE12,
        TXPULSE11 = 0x11 => REG_TXPULSE11,
        TXPULSE10 = 0x12 => REG_TXPULSE10,
        TXPULSE9 = 0x13 => REG_TXPULSE9,
        TXPULSE8 = 0x14 => REG_TXPULSE8,
        TXPULSE7 = 0x15 => REG_TXPULSE7,
        TXPULSE6 = 0x16 => REG_TXPULSE6,
        TXPULSE5 = 0x17 => REG_TXPULSE5,
        TXPULSE4 = 0x18 => REG_TXPULSE4,
        TXPULSE3 = 0x19 => REG_TXPULSE3,
        TXPULSE2 = 0x1A => REG_TXPULSE2,
        TXPULSE1 = 0x1B => REG_TXPULSE1,
        TXPARAMS = 0x1C => REG_TXPARAMS,
        DLLTUNING = 0x1D => REG_DLLTUNING,
        CALIBREQUEST = 0x1E => REG_CALIBREQUEST,
        PWRSTATUS = 0x1F => REG_PWRSTATUS,
        NVMVALUE = 0x20 => REG_NVMVALUE,
        BITTHRES = 0x21 => REG_BITTHRES,
        RSSI = 0x22 => REG_RSSI,
        RNSI = 0x23 => REG_RNSI,
        RXWAITTIME1 = 0x24 => REG_RXWAITTIME1,
        RXWAITTIME0 = 0x25 => REG_RXWAITTIME0,
        FRAMEADDR1 = 0x26 => REG_FRAMEADDR1,
        FRAMEADDR0 = 0x27 => REG_FRAMEADDR0,
        NOISELVL = 0x28 => REG_NOISELVL,
        PREFRAMECFG = 0x29 => REG_PREFRAMECFG,
        MODEMGAIN = 0x2A => REG_MODEMGAIN,
        DEBUGMODEM = 0x2B => REG_DEBUGMODEM,
        MAINMODEM = 0x2C => REG_MAINMODEM,
        CLEARTOSEND = 0x2D => REG_CLEARTOSEND,
        RXPAUSETIME = 0x2E => REG_RXPAUSETIME,
        PREAMBLEN = 0x2F => REG_PREAMBLEN,
        CONSTGAINS = 0x30 => REG_CONSTGAINS,
        SYNCWORDCFG = 0x31 => REG_SYNCWORDCFG,
        SYNCWORD3 = 0x32 => REG_SYNCWORD3,
        SYNCWORD2 = 0x33 => REG_SYNCWORD2,
        SYNCWORD1 = 0x34 => REG_SYNCWORD1,
        SYNCWORD0 = 0x35 => REG_SYNCWORD0,
        CRCPOLYNOM1 = 0x36 => REG_CRCPOLYNOM1,
        CRCPOLYNOM0 = 0x37 => REG_CRCPOLYNOM0,
        REMOTADDR1 = 0x38 => REG_REMOTADDR1,
        REMOTADDR0 = 0x39 => REG_REMOTADDR0,
        LOCALADDR1 = 0x3A => REG_LOCALADDR1,
        LOCALADDR0 = 0x3B => REG_LOCALADDR0,
        TXPKTSIZE = 0x3C => REG_TXPKTSIZE,
        RXPKTSIZE = 0x3D => REG_RXPKTSIZE,
        PACKETCFG = 0x3E => REG_PACKETCFG,
        RXFIFO = 0x3F => REG_RXFIFO
    }

    pub enum Read16Bit {
        STATUS = 0x00,
    }
}

mod write_reg {
    pub enum WriteRegs {
        IRQMASK1 = 0x00,
        IRQMASK2 = 0x01,
        TXFIFOTHRESH = 0x02,
        RXFIFOTHRESH = 0x03,
        SLEEPCONF = 0x04,
        TIMERCONF = 0x05,
        TIMERPERIOD1 = 0x06,
        TIMERPERIOD0 = 0x07,
        RXTIMEOUT1 = 0x08,
        RXTIMEOUT0 = 0x09,
        PWRUPDELAY = 0x0A,
        PLLSTARTUP = 0x0B,
        PLLRATIO = 0x0C,
        RESISTUNE = 0x0D,
        DISABLES = 0x0E,
        RXFILTERS = 0x0F,
        TXPULSE12 = 0x10,
        TXPULSE11 = 0x11,
        TXPULSE10 = 0x12,
        TXPULSE9 = 0x13,
        TXPULSE8 = 0x14,
        TXPULSE7 = 0x15,
        TXPULSE6 = 0x16,
        TXPULSE5 = 0x17,
        TXPULSE4 = 0x18,
        TXPULSE3 = 0x19,
        TXPULSE2 = 0x1A,
        TXPULSE1 = 0x1B,
        TXPARAMS = 0x1C,
        DLLTUNING = 0x1D,
        CALIBRESULT = 0x1E,
        ACTIONS = 0x1F,
        NVMADDRESS = 0x20,
        BITTHRES = 0x21,
        RSSI = 0x22,
        RNSI = 0x23,
        RXWAITTIME1 = 0x24,
        RXWAITTIME0 = 0x25,
        FRAMEADDR1 = 0x26,
        FRAMEADDR0 = 0x27,
        PHSDATA1 = 0x28,
        PHSDATA2 = 0x29,
        PHSDATA3 = 0x2A,
        PHSDATA4 = 0x2B,
        MAINMODEM = 0x2C,
        CLEARTOSEND = 0x2D,
        RXPAUSETIME = 0x2E,
        PREAMBLEN = 0x2F,
        CONSTGAINS = 0x30,
        SYNCWORDCFG = 0x31,
        SYNCWORD3 = 0x32,
        SYNCWORD2 = 0x33,
        SYNCWORD1 = 0x34,
        SYNCWORD0 = 0x35,
        CRCPOLYNOM1 = 0x36,
        CRCPOLYNOM0 = 0x37,
        REMOTADDR1 = 0x38,
        REMOTADDR0 = 0x39,
        LOCALADDR1 = 0x3A,
        LOCALADDR0 = 0x3B,
        TXPKTSIZE = 0x3C,
        RXPKTSIZE = 0x3D,
        PACKETCFG = 0x3E,
        TXFIFO = 0x3F,
    }

    pub enum Write16Bit {
        IRQMASK = 0x00,
    }
}
