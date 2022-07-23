use crate::{commands::Commands, keys::map_string};
use clap::Parser;
use mouse::Mouse;

mod commands;
mod keys;
mod mouse;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(long, short, default_value_t = 0x16C0, value_parser)]
    vid: u16,
    #[clap(long, short, default_value_t = 0x27DD, value_parser)]
    pid: u16,
    #[clap(long, short, default_value_t = 0xFF00, value_parser)]
    usage_page: u16,
    #[clap(long, short, default_value_t = 0x04, value_parser)]
    report_id: u8,
    #[clap(subcommand)]
    command: Commands,
}

fn main() {
    // Get CLI input
    let cli = Cli::parse();
    // Connect to mouse
    let mouse = Mouse::connect(cli.vid, cli.pid, cli.usage_page, cli.report_id);
    // Match command
    match cli.command {
        Commands::SetDpi { dpi } => match dpi {
            Some(dpi) => set_dpi(&mouse, dpi),
            None => read_dpi(&mouse),
        },
        Commands::CpiButtonFunc { command } => match command {
            commands::ButtonOptions::DpiIncrement {
                increment,
                min,
                max,
            } => cpi_dpi_increment(increment, min, max),
            commands::ButtonOptions::KeyPress { keys } => cpi_key(&mouse, keys),
        },
        Commands::LiftButtonFunc { command } => match command {
            commands::ButtonOptions::DpiIncrement {
                increment,
                min,
                max,
            } => lift_dpi_increment(increment, min, max),
            commands::ButtonOptions::KeyPress { keys } => lift_key(&mouse, keys),
        },
        Commands::LoremIpsum => lorem_ipsum(&mouse),
        Commands::Save => save_settings(&mouse),
    };
}

fn lorem_ipsum(mouse: &Mouse) {
    // Get Lorem Ipsum
    let ipsum = mouse.lorem_ipsum();
    // Print it
    println!("As the fables say it goes:\r\n\r\n\t{ipsum}.\r\n\r\nAnd so it was told, by such a thing as a mouse.");
}

fn save_settings(mouse: &Mouse) {
    let result = mouse.save_settings();
    match result {
        Ok(_) => println!("Settings saved!"),
        Err(_) => println!("Failed to save settings"),
    }
}

fn cpi_key(mouse: &Mouse, keys: String) {
    let keys = map_string(keys);
    match keys {
        Ok((mods, keys)) => match mouse.set_cpi_keys(mods, keys) {
            Ok(_) => println!("Keys changed succesfully!"),
            Err(_) => println!("Failed to change keys..."),
        }
        Err(err) => match err {
            keys::ParseError::ToManyKeys(num) => println!("{num} is too many keys, you can only have 6 keys at once, excluding modifiers. Failed to update"),
            keys::ParseError::InvalidToken(token) => println!("{token} is an invalid key. Failed to update"),
        }
    }
}

fn lift_key(mouse: &Mouse, keys: String) {
    let keys = map_string(keys);
    match keys {
        Ok((mods, keys)) => match mouse.set_lift_keys(mods, keys) {
            Ok(_) => println!("Keys changed succesfully!"),
            Err(_) => println!("Failed to change keys..."),
        }
        Err(err) => match err {
            keys::ParseError::ToManyKeys(num) => println!("{num} is too many keys, you can only have 6 keys at once, excluding modifiers. Failed to update"),
            keys::ParseError::InvalidToken(token) => println!("{token} is an invalid key. Failed to update"),
        }
    }
}

fn cpi_dpi_increment(increment: i32, min: u32, max: u32) {
    todo!()
}

fn lift_dpi_increment(increment: i32, min: u32, max: u32) {
    todo!()
}

fn read_dpi(mouse: &Mouse) {
    let dpi = mouse.read_dpi();
    println!("Mouse is set at {dpi} DPI");
}

fn set_dpi(mouse: &Mouse, dpi: u32) {
    mouse.set_dpi(dpi);
    println!("Mouse DPI Set to {}", (dpi / 50) * 50);
}
