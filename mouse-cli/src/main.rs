use crate::{commands::Commands, keys::map_string};
use clap::Parser;
use commands::FunCommand;
use keys::map_keys;
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
        Commands::CpiButtonFunc { keys } => match keys {
            Some(keys) => set_cpi_keys(&mouse, keys),
            None => get_cpi_keys(&mouse),
        },
        Commands::LiftButtonFunc { keys } => match keys {
            Some(keys) => set_lift_keys(&mouse, keys),
            None => get_lift_keys(&mouse),
        },
        Commands::LoremIpsum => lorem_ipsum(&mouse),
        Commands::Save => save_settings(&mouse),
        Commands::Battery => get_battery_voltage(&mouse),
        Commands::SayHi => say_hi(&mouse),
        Commands::Fun { command } => match command {
            FunCommand::AngleSnap { enabled } => match enabled {
                Some(enabled) => set_angle_snap(enabled, &mouse),
                None => get_angle_snap(&mouse),
            },
        },
        Commands::PollingRate { rate } => set_mouse_polling_rate(rate, &mouse),
    };
}

fn set_mouse_polling_rate(rate: u8, mouse: &Mouse) {
    println!("Warning, this function is still really broken, and can only really be used once before you have to reset the mouse (at least on my windows pc).");
    mouse.set_polling_rate(rate);
    println!("Updated polling rate!");
}

fn get_angle_snap(mouse: &Mouse) {
    let val = mouse.get_angle_snap();
    if let Ok(val) = val {
        if val {
            println!("Angle snap is enabled!");
        } else {
            println!("Angle snap is disabled!");
        }
    } else {
        println!("Failed to get angle snap!");
    }
}

fn set_angle_snap(enabled: bool, mouse: &Mouse) {
    if let Ok(()) = mouse.set_angle_snap(enabled) {
        if enabled {
            println!("Angle snap was enabled!");
        } else {
            println!("Angle snap was disabled!");
        }
    } else {
        println!("Failed to set angle snap!");
    }
}

fn say_hi(mouse: &Mouse) {
    let hi = mouse.say_hi();
    match hi {
        Ok(hi) => println!("Mouse: {hi}"),
        Err(_) => println!("Failed to get hi :("),
    }
}

fn get_battery_voltage(mouse: &Mouse) {
    // Get voltage
    let voltage = mouse.get_settings().bat_volt();
    println!("Battery voltage is at {voltage} volts");
}

fn get_lift_keys(mouse: &Mouse) {
    // Get keys
    let keys = mouse.get_settings().lift_keys();
    let mods = mouse.get_settings().lift_mods();
    let string = map_keys(mods, keys);
    println!("Currently Lift keys are set to: {string}");
}

fn get_cpi_keys(mouse: &Mouse) {
    // Get keys
    let keys = mouse.get_settings().cpi_keys();
    let mods = mouse.get_settings().cpi_mods();
    let string = map_keys(mods, keys);
    println!("Currently Cpi keys are set to: {string}");
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

fn set_cpi_keys(mouse: &Mouse, keys: String) {
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

fn set_lift_keys(mouse: &Mouse, keys: String) {
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

fn read_dpi(mouse: &Mouse) {
    let dpi = mouse.get_settings().dpi();
    println!("Mouse is set at {dpi} DPI");
}

fn set_dpi(mouse: &Mouse, dpi: u32) {
    mouse.set_dpi(dpi);
    println!("Mouse DPI Set to {}", (dpi / 50) * 50);
}
