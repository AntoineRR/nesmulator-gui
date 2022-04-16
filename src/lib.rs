use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Instant;
use std::{process::exit, time::Duration};

use env_logger::Env;
use log::{error, info, warn};
use nesmulator_core::{nes::NES, Config};
use sdl2::audio::AudioSpecDesired;
use winit::event_loop::EventLoop;

use crate::gui::Gui;

mod gui;

const DEFAULT_DEBUG_LEVEL: &str = "info";
const MIN_AUDIO_QUEUE_SIZE: u32 = 4 * 4410;

// Different messages that can be thrown at the NES by the event loop
#[derive(PartialEq)]
pub enum Message {
    Input(usize, u8),
    Reset,
    DrawFrame,
    ChangePaletteId(u8),
    ChangeEmulationSpeed(f64),
    SaveState(String),
    Save(String),
    ResizeWindow(u32, u32),
    ToggleDebugWindow,
    CloseApp,
}

pub struct NESConfig<'a> {
    pub rom_path: &'a str,
    pub palette_path: Option<&'a str>,
    pub save_path: &'a str,
    pub state_path: &'a str,
    pub load_state: bool,
    pub debug_level: Option<&'a str>,
    pub display_cpu_logs: bool,
}

pub fn run(nes_config: NESConfig, event_loop: &EventLoop<()>, rx: Receiver<Message>) {
    let config = Config::new(nes_config.palette_path, nes_config.display_cpu_logs);

    init_env_logger(nes_config.debug_level);

    let mut gui = Gui::new(event_loop);

    // Instantiate a NES and connect a ROM file
    let mut nes = NES::from_config(config);
    if nes_config.load_state {
        if let Err(e) = nes.load_state(&nes_config.state_path, nes_config.rom_path) {
            error!("Error parsing state: {}", e);
            exit(1);
        }
        info!("State {} successfully loaded.", nes_config.state_path);
    } else if let Err(e) = nes.insert_cartdrige(nes_config.rom_path) {
        error!("Error parsing ROM: {}", e);
        exit(1);
    }
    info!("ROM {} successfully loaded.", nes_config.rom_path);

    // Load a save for the current cartridge, if any
    if nes.load_save(nes_config.save_path).is_ok() {
        info!("Save successfully loaded.");
    }

    // Spawn a thread to run the NES ROM and give it a channel receiver to handle events from the main loop
    thread::spawn(move || run_nes(&mut nes, &mut gui, rx));
}

fn init_env_logger(debug_level: Option<&str>) {
    let debug_level = if let Some(value) = debug_level {
        match value {
            "0" => "error",
            "1" => "warn",
            "2" => "info",
            "3" => "debug",
            "4" => "trace",
            d => {
                warn!("Invalid debug level : {:?}, value must be in [0;4]. Using default debug level.", d);
                DEFAULT_DEBUG_LEVEL
            }
        }
    } else {
        DEFAULT_DEBUG_LEVEL
    };

    // Setup logger
    // Logs level from winit and pixels crates are set to warn
    env_logger::Builder::from_env(Env::default().default_filter_or(
        debug_level.to_owned()
            + ",gfx_memory=warn,gfx_backend_vulkan=warn,gfx_descriptor=warn,winit=warn,mio=warn,wgpu_core=warn,wgpu_hal=warn,naga=warn",
    ))
    .init();
}

fn run_nes(nes: &mut NES, gui: &mut Gui, rx: Receiver<Message>) {
    info!("Running NES emulation...");

    // Sound
    let sdl_context = sdl2::init().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();

    let desired_audio_specs = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),
        samples: Some(1024),
    };

    let queue = audio_subsystem
        .open_queue(None, &desired_audio_specs)
        .unwrap();
    queue.resume();

    let mut target_time = nes.get_one_frame_duration();
    let mut time = Instant::now();

    loop {
        // Run one clock of emulation
        nes.clock();

        // Handle message from the main thread
        if let Ok(m) = rx.try_recv() {
            let keep_running = handle_message(nes, gui, &mut target_time, m);
            if !keep_running {
                break;
            }
        }

        // Render frame if ready
        if let Some(frame) = nes.get_frame_buffer() {
            gui.update_main_buffer(&frame);
            if gui.debug {
                gui.debug(
                    &nes.get_pattern_table(0).unwrap(),
                    &nes.get_pattern_table(1).unwrap(),
                    &nes.get_palette().unwrap(),
                );
            }
            gui.render().unwrap();

            // Synchronize with sound
            if !nes.is_producing_samples() && queue.size() < MIN_AUDIO_QUEUE_SIZE {
                nes.produce_samples(true);
            } else if nes.is_producing_samples() && queue.size() > MIN_AUDIO_QUEUE_SIZE {
                nes.produce_samples(false);
            }
            queue.queue_audio(&nes.get_samples()[..]).unwrap();

            // Synchronize the emulation to run at the correct speed
            let elapsed_time = time.elapsed();
            if elapsed_time < target_time {
                spin_sleep::sleep(target_time - elapsed_time);
            }
            time = Instant::now();
        }
    }
}

fn handle_message(
    nes: &mut NES,
    gui: &mut Gui,
    target_time: &mut Duration,
    message: Message,
) -> bool {
    match message {
        Message::Input(id, input) => {
            if let Err(e) = nes.input(id, input) {
                error!("Failed to handle controller input: {}", e);
                exit(1);
            }
        }
        Message::Reset => nes.reset(),
        Message::ResizeWindow(width, height) => gui.resize(width, height),
        Message::DrawFrame => gui.redraw(),
        Message::ChangePaletteId(id) => nes.set_debug_palette_id(id).unwrap(),
        Message::ChangeEmulationSpeed(s) => {
            *target_time =
                Duration::from_micros((nes.get_one_frame_duration().as_micros() as f64 / s) as u64)
        }
        Message::SaveState(path) => {
            if let Err(e) = nes.save_state(&path) {
                error!("Failed to save the emulator state: {}", e);
            } else {
                info!("State successfully saved.");
            }
        }
        Message::Save(path) => {
            if let Err(e) = nes.save(&path) {
                error!("Failed to save the game: {}", e);
            } else {
                info!("Game successfully saved at {}.", path);
            }
        }
        Message::ToggleDebugWindow => gui.toggle_debugging(),
        Message::CloseApp => {
            return false;
        }
    }
    true
}
