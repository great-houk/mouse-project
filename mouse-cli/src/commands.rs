use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Reports/Sets the DPI of the Mouse
    #[clap(name = "dpi")]
    SetDpi {
        #[clap(value_parser)]
        dpi: Option<u32>,
    },
    /// Sets the keys to press when the CPI button is pressed.
    #[clap(name = "cpi")]
    CpiButtonFunc { keys: Option<String> },
    /// Sets the keys to press when the mouse is quickly lifted and set back down.
    #[clap(name = "lift")]
    LiftButtonFunc { keys: Option<String> },
    /// Get's the mouse to say back a part of the famous Lorem Ipsum.
    /// Used to check that every device/library is working as intended.
    #[clap(name = "lorem-ipsum")]
    LoremIpsum,
    /// Saves the mouse's current settings (DPI, Keybinds) in to flash memory.
    /// Should be used sparingly, since there is a limited number of times flash can be written to
    #[clap(name = "save")]
    Save,
    /// Reads the battery voltage of the mouse, doesn't work when the mouse is being charged.
    /// Proper voltage should be between 3.0v and 4.2v.
    #[clap(name = "bat")]
    Battery,
    /// Says Hi 😎
    #[clap(name = "sayhi")]
    SayHi,
    /// Sets the number of ms between polls. So 1 means 1000hz polling rate, 2 means 500, etc
    #[clap(name = "poll-rate")]
    PollingRate { rate: u8 },
    /// Unsaveable options for fun :)
    #[clap(name = "fun")]
    Fun {
        #[clap(subcommand)]
        command: FunCommand,
    },
}

#[derive(Debug, Subcommand)]
pub enum FunCommand {
    /// Turns on/off the angle snap mode for the sensor,
    /// making the mouse snap to 5 degree angles. Use with no arguments
    /// to get whether or not it is enabled.
    #[clap(name = "snap")]
    AngleSnap { enabled: Option<bool> },
}
