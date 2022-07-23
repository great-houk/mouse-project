use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Reports or Sets the DPI of the Mouse
    #[clap(name = "dpi")]
    SetDpi {
        #[clap(value_parser)]
        dpi: Option<u32>,
    },
    /// Sets either the keys to press or the DPI to
    /// change by when the CPI button is pressed.
    #[clap(name = "cpi")]
    CpiButtonFunc {
        #[clap(subcommand)]
        command: ButtonOptions,
    },
    /// Sets either the keys to press or the DPI to
    /// change by when the CPI button is pressed.
    #[clap(name = "lift")]
    LiftButtonFunc {
        #[clap(subcommand)]
        command: ButtonOptions,
    },
    /// Get's the mouse to say back a part of the famous Lorem Ipsum.
    /// Used to check that every device/library is working as intended.
    #[clap(name = "lorem-ipsum")]
    LoremIpsum,
    /// Saves the mouse's current settings (DPI, Keybinds) in to flash memory.
    /// Should be used sparingly, since there is a limited number of times flash can be written to
    #[clap(name = "save")]
    Save,
}

#[derive(Debug, Subcommand)]
pub enum ButtonOptions {
    /// Set button to increase/decrease the DPI by an increment,
    /// and also what the max DPI is before wrapping to min.
    /// Mouse increments in steps of 50, so everything not divisble by 50 will
    /// be rounded, and the min and max DPI possible are 50 and 16,000 respectively.
    #[clap(name = "dpi")]
    DpiIncrement {
        #[clap(value_parser)]
        increment: i32,
        #[clap(value_parser, default_value_t = 500)]
        min: u32,
        #[clap(value_parser, default_value_t = 5000)]
        max: u32,
    },
    /// Set button to act as up to 6 keys plus modifiers (shift, ctrl, etc...)
    /// seperated by plus signs. You can also enter in hex values in the
    /// form 0x__+0x__ where the underscores would be hex digits. The values correspond to the
    /// USB HID Keyboard keycodes. For example, F13 is 0x68 so control + F13 would be
    /// CTRL+0x68
    #[clap(name = "key")]
    KeyPress {
        #[clap(value_parser)]
        keys: String,
    },
}
