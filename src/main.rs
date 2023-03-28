use std::cmp::Ordering;

use clap::*;
use pixels::{Pixels, SurfaceTexture};
use thiserror::Error;
use winit::{
        dpi::PhysicalSize,
        error::OsError,
        event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
};

#[derive(Debug, Error)]
enum RVUError {
        #[error("Unable to create a window.")]
        WindowError(#[from] OsError),

        #[error("An error occured while processing the image.")]
        ImageError(#[from] image::ImageError),

        #[error("An error  occured while loading the image.")]
        IoError(#[from] std::io::Error),

        #[error("Unable to calculate maximum screen size on your primary monitor.")]
        NoPrimaryMonitor,

        #[error("Couldn't draw the requested image.")]
        PixelsError(#[from] pixels::Error),

        #[error("Couldn't render the requested image.")]
        RenderError(pixels::Error),
}

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Config {
        /// Name of the image to view
        file_name: String,
}

type Result<T> = std::result::Result<T, RVUError>;

const SCREEN_PERCENT: u32 = 90;

fn main() -> Result<()> {
        let config = Config::parse();
        let img = image::io::Reader::open(&config.file_name)?.decode()?;

        let event_loop = EventLoop::new();
        let primary_monitor = event_loop
                .primary_monitor()
                .ok_or(RVUError::NoPrimaryMonitor)?;

        let screen_size = primary_monitor.size();
        let max_screen_size = (
                screen_size.width * SCREEN_PERCENT / 100,
                screen_size.height * SCREEN_PERCENT / 100,
        );

        let hscale = calc_scale(max_screen_size.0, img.width());
        let vscale = calc_scale(max_screen_size.1, img.height());
        let scale = std::cmp::max(hscale, vscale);
        let window_inner_size = PhysicalSize::new(img.width() / scale, img.height() / scale);

        let window = WindowBuilder::new()
                .with_title("RVU")
                .with_inner_size(window_inner_size)
                .build(&event_loop)?;

        let surface =
                SurfaceTexture::new(window_inner_size.width, window_inner_size.height, &window);
        let mut pixels = Pixels::new(img.width(), img.height(), surface)?;
        let img_bytes = img.as_rgb8().unwrap().as_flat_samples();
        let img_bytes = img_bytes.as_slice();
        let pxl_bytes = pixels.frame_mut();

        img_bytes
                .chunks_exact(3)
                .zip(pxl_bytes.chunks_exact_mut(4))
                .for_each(|(source, target)| {
                        target[0] = source[0];
                        target[1] = source[1];
                        target[2] = source[2];
                        target[3] = 0xff;
                });

        event_loop.run(move |ev, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match ev {
            Event::RedrawRequested(_) => { let _ = pixels.render().map_err(RVUError::RenderError); },
            Event::WindowEvent { window_id, event } if window_id == window.id() => match event {
                WindowEvent::ScaleFactorChanged { scale_factor: _, new_inner_size } => resize(&mut pixels, &new_inner_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => resize(&mut pixels, &size),
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
               _ => (),
            },
            _ => (),
        }
    });
}

fn calc_scale(max_size: u32, size: u32) -> u32 {
        match max_size.cmp(&size) {
                Ordering::Greater => 1,
                Ordering::Equal => 0,
                Ordering::Less => (size as f32 / max_size as f32).ceil() as u32,
        }
}

fn resize(pxls: &mut Pixels, size: &PhysicalSize<u32>) {
        let _ = pxls.resize_surface(size.width, size.height);
}
