extern crate sdl2;
extern crate palette;

use sdl2::EventPump;
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Renderer;
use sdl2::render::Texture;
use sdl2::pixels::Color;

use palette::pixel::GammaRgb;

const SCREEN_WIDTH: usize = 1024;
const SCREEN_HEIGHT: usize = 400;
const COLOR_BYTES: usize = 3;
const LINE_SIZE: usize = COLOR_BYTES * SCREEN_WIDTH;

const BAR_PXL_WIDTH: usize = 4;
const BAR_HEIGHT: usize = 100;

pub struct Repeater<I : Iterator> {
    repeats: usize,
    counter: usize,
    item: Option<I::Item>,
    iter: I
}

impl<I : Iterator> Repeater<I> {
    pub fn new(iter: I, repeats: usize) -> Repeater<I> {
        Repeater {
            repeats: repeats,
            counter: 0,
            item: None,
            iter: iter
        }
    }
}

impl<I : Iterator> Iterator for Repeater<I> where I::Item: Clone {
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> {
        //restart counter and get next item
        if self.counter == 0 {
            self.counter = self.repeats;
            self.item = self.iter.next();
        }

        self.counter -= 1;

        //return orginal item on last repeat
        if self.counter == 0 {
            return self.item.take();
        }

        //return copy of current item
        match self.item {
            Some(ref item) => Some(item.clone()),
            None => None,
        }
    }
}

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

fn draw(renderer: &mut Renderer, texture: &mut Texture, gamma: f32){
    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();

    let lookup: Vec<u8> = (0..256).map(|i| {
        let j = i as u8;
        let c = GammaRgb::new_u8(j, j, j, gamma).to_linear();
        let pxl: [u8; COLOR_BYTES] = c.to_pixel();
        pxl[0]
    }).collect();

    texture.with_lock(None, |buffer: &mut [u8], _| {
        let clines = buffer.chunks_mut(LINE_SIZE * BAR_HEIGHT);
        for cline in clines.zip(0..4) {
            for line in cline.0.chunks_mut(LINE_SIZE).enumerate() {
                let rep = Repeater::new(0..256, BAR_PXL_WIDTH);
                for pxl in line.1.chunks_mut(COLOR_BYTES).zip(rep) {
                    let c = lookup[pxl.1];
                    match cline.1 {
                        0 => pxl.0.clone_from_slice(&[c, c, c]),
                        1 => pxl.0.clone_from_slice(&[c, 0, 0]),
                        2 => pxl.0.clone_from_slice(&[0, c, 0]),
                        3 => pxl.0.clone_from_slice(&[0, 0, c]),
                        _ => {}
                    }
                }
            }
        }
    }).unwrap();
    renderer.copy(&texture, None, None);
    renderer.present();
}

fn main() {
    if (BAR_PXL_WIDTH * 256 > SCREEN_WIDTH) ||  (BAR_HEIGHT * 4 > SCREEN_HEIGHT) {
        panic!("Invalid resolution");
    }

    let sdl    = sdl2::init().unwrap();
    let video  = sdl.video().unwrap();
    let window = video.window("Color Palette SDL2", 
        SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut gamma = 1.0;
    let mut renderer = window.renderer().build().unwrap();
    let mut texture = renderer.create_texture_streaming(
        PixelFormatEnum::RGB24, SCREEN_WIDTH as u32, 
        SCREEN_HEIGHT as u32).unwrap();

    let mut event_pump = sdl.event_pump().unwrap();
    while !handle_events(&mut event_pump, &mut gamma) {
        draw(&mut renderer, &mut texture, gamma);
    }
}
