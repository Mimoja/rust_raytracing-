mod camera;
mod hittable;
mod ray;
mod raytrace;
mod scene;
mod material;

use instant::Instant;
use raytrace::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use image::*;

fn main() {
    let nx = 1920/2;
    let ny = 1080/2;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Rust raytracer", nx, ny)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, nx, ny)
        .map_err(|e| e.to_string())
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0.0;
    let now = Instant::now();
    let mut sample = 0;
    let mut raycaster = RayCaster::new(nx as usize, ny as usize);

    'running: loop {
        i += 0.05;
        sample += 1;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::O),
                    ..
                } => {
                    raycaster.orthonogal = !raycaster.orthonogal;
                    raycaster.clearImage();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    let mut arr = vec![0; (nx*ny*3) as usize];
                    let buffer = arr.as_mut_slice();
                    raycaster.fill_buffer(buffer);
                    image::save_buffer("image.png", buffer, nx, ny, image::RGB(8)).unwrap();
                    println!("Image saved")
                }
                _ => {}
            }
        }
        texture
            .with_lock(None, |buffer: &mut [u8], _pitch: usize| {
                raycaster.trace(i);
                raycaster.fill_buffer(buffer);
                println!(
                    "Rendering took {} seconds for {} sample",
                    now.elapsed().as_secs(),
                    sample
                );
            })
            .unwrap();

        canvas.clear();
        canvas
            .copy(&texture, None, Some(Rect::new(0, 0, nx, ny)))
            .unwrap();
        canvas.present();
    }
}
