use crate::{commands::Commands, keys::map_string};
use clap::Parser;
use commands::{FunCommand, RecordCommand};
use keys::map_keys;
use mouse::Mouse;
use pixels::{Pixels, SurfaceTexture};
use std::{fs::File, time::Instant};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;
use y4m::{Frame, Ratio};

mod commands;
mod keys;
mod mouse;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(value_parser)]
    name: String,
    #[clap(subcommand)]
    command: Commands,
}

fn main() {
    // Get CLI input
    // let cli = Cli::parse();
    let cli = Cli {
        name: "COM9".to_string(),
        command: Commands::Battery,
    };
    // Connect to mouse
    let mut mouse = Mouse::connect(cli.name).unwrap();
    // Match command
    match cli.command {
        Commands::SetDpi { dpi } => match dpi {
            Some(dpi) => set_dpi(&mut mouse, dpi),
            None => read_dpi(&mut mouse),
        },
        Commands::CpiButtonFunc { keys } => match keys {
            Some(keys) => set_cpi_keys(&mut mouse, keys),
            None => get_cpi_keys(&mut mouse),
        },
        Commands::LiftButtonFunc { keys } => match keys {
            Some(keys) => set_lift_keys(&mut mouse, keys),
            None => get_lift_keys(&mut mouse),
        },
        Commands::LoremIpsum => lorem_ipsum(&mut mouse),
        Commands::Save => save_settings(&mut mouse),
        Commands::Battery => get_battery_voltage(&mut mouse),
        Commands::SayHi => say_hi(&mut mouse),
        Commands::Fun { command } => match command {
            FunCommand::AngleSnap { enabled } => match enabled {
                Some(enabled) => set_angle_snap(enabled, &mut mouse),
                None => get_angle_snap(&mut mouse),
            },
            FunCommand::Record { command } => match command {
                RecordCommand::Record { path, frames } => {
                    record_mouse_frames(path, frames, &mut mouse)
                }
                RecordCommand::Live => display_mouse_frames(mouse),
            },
        },
        Commands::PollingRate { rate } => set_mouse_polling_rate(rate, &mut mouse),
    };
}

fn display_mouse_frames(mut mouse: Mouse) {
    // Start frame reading
    mouse.start_frame_read(0).unwrap();
    let mut done = false;
    // Make window
    const MULT: usize = 20;
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new((36 * MULT) as f64, (36 * MULT) as f64);
        WindowBuilder::new()
            .with_title("Mouse Vision")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(36 * MULT as u32, 36 * MULT as u32, surface_texture).unwrap()
    };
    // Run even loop
    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            // Get the current frame
            let frame_data = mouse.read_frame().unwrap();
            // Update frame buffer
            let frame_buffer = pixels.get_frame().chunks_exact_mut(4).enumerate();
            for (i, rgb) in frame_buffer {
                let x = 35 - (i / (36 * MULT)) / MULT;
                let y = (i % (36 * MULT)) / MULT;
                let f_ind = x + y * 36;
                rgb[0] = frame_data[f_ind];
                rgb[1] = frame_data[f_ind];
                rgb[2] = frame_data[f_ind];
                rgb[3] = 255;
            }
            // Render
            if pixels
                .render()
                .map_err(|e| dbg!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Request a redraw
            window.request_redraw();
        }

        // Handle exit
        if !done && *control_flow == ControlFlow::Exit {
            // Stop Mouse
            mouse.end_frame_read().unwrap();
            // Print
            println!("Done!");
            done = true;
        }
    });
}

fn record_mouse_frames(path: String, iters: u16, mouse: &mut Mouse) {
    // Start frame reading
    let now = Instant::now();
    mouse.start_frame_read(iters).unwrap();
    // Generate Frames
    let mut frames = Vec::new();
    for _ in 0..iters {
        let frame_data = mouse.read_frame().unwrap();
        frames.push(frame_data);
    }
    // Finish
    mouse.end_frame_read().unwrap();
    // Get FPS
    let time = now.elapsed().as_millis();
    let fps = f32::round((iters as f32 * 1000.0) / time as f32) as usize;
    // Make file
    let file = File::create(format!("./{path}")).unwrap();
    // Start encoding
    let mut enc = y4m::encode(36, 36, Ratio::new(fps, 1))
        .with_colorspace(y4m::Colorspace::Cmono)
        .write_header(file)
        .unwrap();
    // Encode
    for frame_data in frames {
        let frame = Frame::new([&frame_data[..], &[], &[]], None);
        enc.write_frame(&frame).unwrap();
    }
    // Done
    println!("Done! Read {iters} frames into ./{path} at {fps}fps");
}

fn set_mouse_polling_rate(rate: u8, mouse: &mut Mouse) {
    println!("Warning, this function is still really broken, and can only really be used once before you have to reset the mouse (at least on my windows pc).");
    mouse.set_polling_rate(rate);
    println!("Updated polling rate!");
}

fn get_angle_snap(mouse: &mut Mouse) {
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

fn set_angle_snap(enabled: bool, mouse: &mut Mouse) {
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

fn say_hi(mouse: &mut Mouse) {
    let hi = mouse.say_hi();
    match hi {
        Ok(hi) => println!("Mouse: {hi}"),
        Err(_) => println!("Failed to get hi :("),
    }
}

fn get_battery_voltage(mouse: &mut Mouse) {
    // Get voltage
    let voltage = mouse.get_settings().bat_volt();
    println!("Battery voltage is at {voltage} volts");
}

fn get_lift_keys(mouse: &mut Mouse) {
    // Get keys
    let keys = mouse.get_settings().lift_keys();
    let mods = mouse.get_settings().lift_mods();
    let string = map_keys(mods, keys);
    println!("Currently Lift keys are set to: {string}");
}

fn get_cpi_keys(mouse: &mut Mouse) {
    // Get keys
    let keys = mouse.get_settings().cpi_keys();
    let mods = mouse.get_settings().cpi_mods();
    let string = map_keys(mods, keys);
    println!("Currently Cpi keys are set to: {string}");
}

fn lorem_ipsum(mouse: &mut Mouse) {
    // Get Lorem Ipsum
    let ipsum = mouse.lorem_ipsum();
    // Print it
    println!("As the fables say it goes:\r\n\r\n\t{ipsum}.\r\n\r\nAnd so it was told, by such a thing as a mouse.");
}

fn save_settings(mouse: &mut Mouse) {
    let result = mouse.save_settings();
    match result {
        Ok(_) => println!("Settings saved!"),
        Err(_) => println!("Failed to save settings"),
    }
}

fn set_cpi_keys(mouse: &mut Mouse, keys: String) {
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

fn set_lift_keys(mouse: &mut Mouse, keys: String) {
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

fn read_dpi(mouse: &mut Mouse) {
    let dpi = mouse.get_settings().dpi();
    println!("Mouse is set at {dpi} DPI");
}

fn set_dpi(mouse: &mut Mouse, dpi: u32) {
    mouse.set_dpi(dpi);
    println!("Mouse DPI Set to {}", (dpi / 50) * 50);
}
