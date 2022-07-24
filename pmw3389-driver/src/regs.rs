use crate::{pmw3389::Register, Pmw3389Error};

pub(crate) enum RegisterSize {
    Single(Register),
    DoubleHighFirst(Register, Register),
    DoubleLowFirst(Register, Register),
}

pub enum Pmw3389Register {
    ProductId,
    RevisionId,
    Motion,
    Squal,
    RawDataSum,
    MaximumRawData,
    MinimumRawData,
    Shutter,
    RippleControl,
    Resolution,
    Config2,
    AngleTune,
    RunDownshift,
    Rest1Rate,
    Rest1Downshift,
    Rest2Rate,
    Rest2Downshift,
    Rest3Rate,
    Observation,
    SromId,
    MinSQRun,
    RawDataThreshold,
    Control2,
    Config5,
    PowerUpReset,
    Shutdown,
    InverseProductID,
    LiftCutoffCal3,
    AngleSnap,
    LiftCutoffCal1,
    LiftConfig,
    LiftCutoffCal2,
    LiftCutoffCalTimeout,
    LiftCutoffCalMinLength,
    PWMPeriodCnt,
    PWMWidthCnt,
}

impl Pmw3389Register {
    pub(crate) fn to_register_read(self) -> Result<RegisterSize, Pmw3389Error> {
        match self {
            Self::PowerUpReset | Self::Shutdown => Err(Pmw3389Error::CantRead),
            _ => self.to_register(),
        }
    }

    pub(crate) fn to_register_write(self) -> Result<RegisterSize, Pmw3389Error> {
        match self {
            Self::ProductId
            | Self::RevisionId
            | Self::Squal
            | Self::RawDataSum
            | Self::MaximumRawData
            | Self::MinimumRawData
            | Self::Shutter
            | Self::SromId
            | Self::InverseProductID
            | Self::LiftCutoffCal2 => Err(Pmw3389Error::CantWrite),
            _ => self.to_register(),
        }
    }

    fn to_register(self) -> Result<RegisterSize, Pmw3389Error> {
        Ok(match self {
            Pmw3389Register::ProductId => RegisterSize::Single(Register::ProductId),
            Pmw3389Register::RevisionId => RegisterSize::Single(Register::RevisionId),
            Pmw3389Register::Motion => RegisterSize::Single(Register::Motion),
            Pmw3389Register::Squal => RegisterSize::Single(Register::Squal),
            Pmw3389Register::RawDataSum => RegisterSize::Single(Register::RawDataSum),
            Pmw3389Register::MaximumRawData => RegisterSize::Single(Register::MaximumRawData),
            Pmw3389Register::MinimumRawData => RegisterSize::Single(Register::MinimumRawData),
            Pmw3389Register::Shutter => {
                RegisterSize::DoubleHighFirst(Register::ShutterLower, Register::ShutterUpper)
            }
            Pmw3389Register::RippleControl => RegisterSize::Single(Register::RippleControl),
            Pmw3389Register::Resolution => {
                RegisterSize::DoubleHighFirst(Register::ResolutionL, Register::ResolutionH)
            }
            Pmw3389Register::Config2 => RegisterSize::Single(Register::Config2),
            Pmw3389Register::AngleTune => RegisterSize::Single(Register::AngleTune),
            Pmw3389Register::RunDownshift => RegisterSize::Single(Register::RunDownshift),
            Pmw3389Register::Rest1Rate => {
                RegisterSize::DoubleLowFirst(Register::Rest1RateLower, Register::Rest1RateUpper)
            }
            Pmw3389Register::Rest1Downshift => RegisterSize::Single(Register::Rest1Downshift),
            Pmw3389Register::Rest2Rate => {
                RegisterSize::DoubleLowFirst(Register::Rest2RateLower, Register::Rest2RateUpper)
            }
            Pmw3389Register::Rest2Downshift => RegisterSize::Single(Register::Rest2Downshift),
            Pmw3389Register::Rest3Rate => {
                RegisterSize::DoubleLowFirst(Register::Rest3RateLower, Register::Rest3RateUpper)
            }
            Pmw3389Register::Observation => RegisterSize::Single(Register::Observation),
            Pmw3389Register::SromId => RegisterSize::Single(Register::SromId),
            Pmw3389Register::MinSQRun => RegisterSize::Single(Register::MinSQRun),
            Pmw3389Register::RawDataThreshold => RegisterSize::Single(Register::RawDataThreshold),
            Pmw3389Register::Control2 => RegisterSize::Single(Register::Control2),
            Pmw3389Register::Config5 => {
                RegisterSize::DoubleHighFirst(Register::Config5L, Register::Config5H)
            }
            Pmw3389Register::PowerUpReset => RegisterSize::Single(Register::PowerUpReset),
            Pmw3389Register::Shutdown => RegisterSize::Single(Register::Shutdown),
            Pmw3389Register::InverseProductID => RegisterSize::Single(Register::InverseProductID),
            Pmw3389Register::LiftCutoffCal3 => RegisterSize::Single(Register::LiftCutoffCal3),
            Pmw3389Register::AngleSnap => RegisterSize::Single(Register::AngleSnap),
            Pmw3389Register::LiftCutoffCal1 => RegisterSize::Single(Register::LiftCutoffCal1),
            Pmw3389Register::LiftConfig => RegisterSize::Single(Register::LiftConfig),
            Pmw3389Register::LiftCutoffCal2 => RegisterSize::Single(Register::LiftCutoffCal2),
            Pmw3389Register::LiftCutoffCalTimeout => {
                RegisterSize::Single(Register::LiftCutoffCalTimeout)
            }
            Pmw3389Register::LiftCutoffCalMinLength => {
                RegisterSize::Single(Register::LiftCutoffCalMinLength)
            }
            Pmw3389Register::PWMPeriodCnt => RegisterSize::Single(Register::PWMPeriodCnt),
            Pmw3389Register::PWMWidthCnt => RegisterSize::Single(Register::PWMWidthCnt),
        })
    }
}
