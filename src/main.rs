use std::path::Path;
use std::process::exit;
use std::sync::mpsc;

use clap::{Arg, Command};
use log::{error, info};
use nesmulator_core::utils::ControllerInput;
use nesmulator_gui::{run, Message, NESConfig};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;

fn main() {
    // CLI creation
    let matches = Command::new("Nesmulator")
        .version("0.1.0")
        .author("AntoineRR")
        .about("nesmulator-gui - CLI to launch a GUI based on the nesmulator-core crate")
        .arg(
            Arg::new("game")
                .index(1)
                .value_name("FILE")
                .help("Sets the nes file to run in the emulator")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .value_name("INT")
                .takes_value(true)
                .help("Turn debugging information on"),
        )
        .arg(
            Arg::new("log")
                .short('l')
                .long("log")
                .help("Display the CPU logs to the console"),
        )
        .arg(
            Arg::new("palette")
                .short('p')
                .long("palette")
                .value_name("FILE")
                .takes_value(true)
                .help("Sets a palette from a .pal file"),
        )
        .arg(
            Arg::new("state")
                .short('s')
                .long("state")
                .value_name("FILE")
                .takes_value(true)
                .help("Specify a .data state file to load in the emulator"),
        )
        .get_matches();

    // Get all configuration informations
    let palette_path = matches.value_of("palette");
    let state_path = matches.value_of("state");
    let display_cpu_logs = matches.is_present("log");
    let debug_level = matches.value_of("debug");
    let rom_path = matches.value_of("game").unwrap();

    // Create the GUI for displaying the graphics
    let event_loop = EventLoop::new();
    let (tx, rx): (mpsc::Sender<Message>, mpsc::Receiver<Message>) = mpsc::channel();

    run(
        NESConfig {
            rom_path,
            palette_path,
            state_path,
            debug_level,
            display_cpu_logs,
        },
        &event_loop,
        rx,
    );

    // Run the event loop
    let mut palette_id = 0;
    let mut speed = 1.0;
    let path_to_rom = Path::new(rom_path);
    let path_to_state = path_to_rom
        .parent()
        .unwrap()
        .join(path_to_rom.file_stem().unwrap())
        .with_extension("data");
    let state_path = String::from(path_to_state.to_str().unwrap());
    let mut input_helper = WinitInputHelper::new();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::RedrawRequested(_) = event {
            send_message(&tx, Message::DrawFrame, control_flow);
        }

        if input_helper.update(&event) {
            // Close event
            if input_helper.key_pressed(VirtualKeyCode::Escape) || input_helper.quit() {
                *control_flow = ControlFlow::Exit;
                send_message(&tx, Message::CloseApp, control_flow);
                info!("Closing application...");
                exit(0);
            }
            // Resize event
            if let Some(size) = input_helper.window_resized() {
                send_message(
                    &tx,
                    Message::ResizeWindow(size.width, size.height),
                    control_flow,
                );
            }
            // Debug window
            if input_helper.key_pressed(VirtualKeyCode::E) {
                send_message(&tx, Message::ToggleDebugWindow, control_flow);
            }
            // Reset
            if input_helper.key_pressed(VirtualKeyCode::R) {
                send_message(&tx, Message::Reset, control_flow);
            }
            // Change debug palette
            if input_helper.key_pressed(VirtualKeyCode::Left) {
                if palette_id == 0 {
                    palette_id = 7;
                } else {
                    palette_id -= 1;
                }
                send_message(&tx, Message::ChangePaletteId(palette_id), control_flow);
            }
            if input_helper.key_pressed(VirtualKeyCode::Right) {
                if palette_id == 7 {
                    palette_id = 0;
                } else {
                    palette_id += 1;
                }
                send_message(&tx, Message::ChangePaletteId(palette_id), control_flow);
            }
            // Change emulation speed
            if input_helper.key_pressed(VirtualKeyCode::Up) {
                speed += 0.5;
                send_message(&tx, Message::ChangeEmulationSpeed(speed), control_flow);
            }
            if input_helper.key_pressed(VirtualKeyCode::Down) {
                speed -= 0.5;
                send_message(&tx, Message::ChangeEmulationSpeed(speed), control_flow);
            }
            // Save state
            if input_helper.key_pressed(VirtualKeyCode::M) {
                send_message(&tx, Message::SaveState(state_path.clone()), control_flow);
            }
            // Controller inputs
            let mut input = 0;
            if input_helper.key_held(VirtualKeyCode::Z) {
                input |= ControllerInput::Up as u8;
            }
            if input_helper.key_held(VirtualKeyCode::Q) {
                input |= ControllerInput::Left as u8;
            }
            if input_helper.key_held(VirtualKeyCode::S) {
                input |= ControllerInput::Down as u8;
            }
            if input_helper.key_held(VirtualKeyCode::D) {
                input |= ControllerInput::Right as u8;
            }
            if input_helper.key_held(VirtualKeyCode::X) {
                input |= ControllerInput::Start as u8;
            }
            if input_helper.key_held(VirtualKeyCode::C) {
                input |= ControllerInput::Select as u8;
            }
            if input_helper.key_held(VirtualKeyCode::I) {
                input |= ControllerInput::A as u8;
            }
            if input_helper.key_held(VirtualKeyCode::O) {
                input |= ControllerInput::B as u8;
            }
            send_message(&tx, Message::Input(0, input), control_flow);
        }
    });
}

fn send_message(tx: &mpsc::Sender<Message>, message: Message, control_flow: &mut ControlFlow) {
    if tx.send(message).is_err() {
        error!("Receiving thread 'run_nes' panicked");
        *control_flow = ControlFlow::Exit;
        exit(1);
    }
}
