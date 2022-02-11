use std::error::Error;

use winit::window::{Window, WindowBuilder};
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use pixels::{Pixels, SurfaceTexture};

use nesmulator_core::utils::ARGBColor;

const MAIN_WINDOW_WIDTH: u32 = 256;
pub const MAIN_WINDOW_HEIGHT: u32 = 240;

pub const DEBUG_WINDOW_WIDTH: u32 = 256;
pub const DEBUG_WINDOW_HEIGHT: u32 = 240 + 2 + 128 + 2 + 6; // From top to bottom: main window | pattern table | palette

#[derive(Debug)]
pub struct GUI {
    main_window: Window,
    main_pixels: Pixels,
    pub debug: bool,
}

impl GUI {
    pub fn new(main_event_loop: &EventLoop<()>) -> Self {
        let window_size = LogicalSize::new(MAIN_WINDOW_WIDTH * 2, MAIN_WINDOW_HEIGHT * 2);
        let buffer_size = LogicalSize::new(MAIN_WINDOW_WIDTH, MAIN_WINDOW_HEIGHT);
        let main_window = WindowBuilder::new()
            .with_title("Nesmulator")
            .with_inner_size(window_size)
            .with_min_inner_size(buffer_size)
            .build(main_event_loop)
            .expect("Cannot create main window");

        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, &main_window);
        let main_pixels =
            Pixels::new(buffer_size.width, buffer_size.height, surface_texture).unwrap();

        GUI {
            main_window,
            main_pixels,
            debug: false,
        }
    }

    pub fn toggle_debugging(&mut self) {
        if self.debug {
            let width = MAIN_WINDOW_WIDTH;
            let height = MAIN_WINDOW_HEIGHT;
            self.main_pixels.resize_buffer(width, height);
            self.debug = false;
        } else {
            let width = DEBUG_WINDOW_WIDTH;
            let height = DEBUG_WINDOW_HEIGHT;
            self.main_pixels.resize_buffer(width, height);
            self.debug = true;
        }
    }

    fn add_pattern_tables(
        &mut self,
        buffer: &mut [ARGBColor],
        pattern_table_0: &[ARGBColor],
        pattern_table_1: &[ARGBColor],
    ) {
        for (i, color) in pattern_table_0.iter().enumerate() {
            buffer[(i / 128) * 256 + i % 128] = *color;
        }
        for (i, color) in pattern_table_1.iter().enumerate() {
            buffer[(i / 128) * 256 + i % 128 + 128] = *color;
        }
    }

    fn add_separation(&mut self, buffer: &mut [ARGBColor]) {
        for i in 0..512 {
            buffer[i] = ARGBColor::light_gray();
        }
    }

    fn add_palette(&mut self, buffer: &mut [ARGBColor], palette: &[ARGBColor]) {
        for (offset, color) in palette.iter().enumerate() {
            for i in 0..6 {
                for j in 0..6 {
                    let index = (offset * 6) + (((offset % 4) == 0) as usize) * 2 + i + j * 256;
                    buffer[index] = *color;
                }
            }
        }
    }

    pub fn debug(
        &mut self,
        pattern_table_0: &[ARGBColor],
        pattern_table_1: &[ARGBColor],
        palette: &[ARGBColor],
    ) {
        const BUFFER_SIZE: usize =
            ((DEBUG_WINDOW_HEIGHT - MAIN_WINDOW_HEIGHT) * DEBUG_WINDOW_WIDTH) as usize;
        let mut buffer = [ARGBColor::black(); BUFFER_SIZE];
        let mut offset = 0;
        self.add_separation(&mut buffer[offset..offset + 512]);
        offset += 512;
        self.add_pattern_tables(
            &mut buffer[offset..offset + 32768],
            pattern_table_0,
            pattern_table_1,
        );
        offset += 32768;
        self.add_separation(&mut buffer[offset..offset + 512]);
        offset += 512;
        self.add_palette(&mut buffer[offset..offset + 1536], palette);

        self.update_debug_buffer(&buffer);
    }

    pub fn update_main_buffer(&mut self, buffer: &[ARGBColor; 61_440]) {
        for (i, color) in buffer.iter().enumerate() {
            self.update_pixel(i, color);
        }
    }

    fn update_debug_buffer(&mut self, buffer: &[ARGBColor]) {
        let offset = (MAIN_WINDOW_WIDTH * MAIN_WINDOW_HEIGHT) as usize;
        for (i, color) in buffer.iter().enumerate() {
            self.update_pixel(offset + i, color);
        }
    }

    pub fn redraw(&mut self) {
        self.main_window.request_redraw();
    }

    pub fn render(&mut self) -> Result<(), Box<dyn Error>> {
        self.main_pixels.render()?;
        Ok(())
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.main_pixels.resize_surface(width, height);
    }

    fn update_pixel(&mut self, offset: usize, color: &ARGBColor) {
        let pixel = &mut self.main_pixels.get_frame()[offset * 4..offset * 4 + 4];
        pixel[0] = color.red;
        pixel[1] = color.green;
        pixel[2] = color.blue;
        pixel[3] = color.alpha;
    }
}