extern crate sdl2;

use sdl2::EventPump;
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::video::Window;
use sdl2::pixels::Color;

use std::iter;

const SCREEN_WIDTH: usize = 1024;
const SCREEN_HEIGHT: usize = 400;
const COLOR_BYTES: usize = 3;
const LINE_SIZE: usize = COLOR_BYTES * SCREEN_WIDTH;

const BAR_PXL_WIDTH: usize = 4;
const BAR_HEIGHT: usize = 100;

fn handle_events(event_pump: &mut EventPump, gamma: &mut f32) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return true;
            },
            Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                *gamma += 0.05;
                println!("Gamma:{}", *gamma);
            },
            Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                *gamma -= 0.05;
                if *gamma < 0.0 {
                    *gamma = 0.0;
                }
                println!("Gamma:{}", *gamma);
            },
            _ => {}
        }
    }
    false
}

fn draw(canvas: &mut Canvas<Window>, texture: &mut Texture, gamma: f32){
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let lookup: Vec<u8> = (0usize..256).map(|i| {
        let mut g = 255.0 * ((i as f32) / 255.0).powf(gamma);
        if g < 0.0 { g = 0.0; }
        if g > 255.0 { g = 255.0; }
        g as u32 as u8
    }).collect();

    texture.with_lock(None, |buffer: &mut [u8], _| {
        let clines = buffer.chunks_mut(LINE_SIZE * BAR_HEIGHT);
        for cline in clines.zip(0..4) {
            for line in cline.0.chunks_mut(LINE_SIZE).enumerate() {
                for pxl in line.1.chunks_mut(COLOR_BYTES).zip((0..256usize)
                    .flat_map(|i| iter::repeat(i).take(BAR_PXL_WIDTH)))
                {
                    let c = lookup[pxl.1 as usize];
                    match cline.1 {
                        0 => pxl.0.clone_from_slice(&[c, c, c]),
                        1 => pxl.0.clone_from_slice(&[c, 0, 0]),
                        2 => pxl.0.clone_from_slice(&[0, c, 0]),
                        3 => pxl.0.clone_from_slice(&[0, 0, c]),
                        _ => {}
                    }

                    let l = cline.1 * BAR_HEIGHT + line.0;
                    let p = (((255 - c) as f32 / 255.0) * (4 * BAR_HEIGHT) as f32) as usize;
                    if l == p {
                        for c in pxl.0.iter_mut() {
                            *c = std::u8::MAX - *c;
                        }
                    }
                }
            }
        }
    }).unwrap();
    canvas.copy(&texture, None, None).unwrap();
    canvas.present();
}

fn main() {
    if (BAR_PXL_WIDTH * 256 != SCREEN_WIDTH) ||  (BAR_HEIGHT * 4 != SCREEN_HEIGHT) {
        panic!("Invalid resolution");
    }

    let sdl    = sdl2::init().unwrap();
    let video  = sdl.video().unwrap();
    let window = video.window("gamma-correction-demo", 
        SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut gamma = 1.0;
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(
        PixelFormatEnum::RGB24, SCREEN_WIDTH as u32, 
        SCREEN_HEIGHT as u32).unwrap();

    let mut event_pump = sdl.event_pump().unwrap();
    while !handle_events(&mut event_pump, &mut gamma) {
        draw(&mut canvas, &mut texture, gamma);
    }
}
