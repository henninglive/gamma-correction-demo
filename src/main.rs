extern crate sdl2;
extern crate palette;

use sdl2::EventPump;
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Renderer;
use sdl2::render::Texture;

use palette::pixel::GammaRgb;

const SCREEN_WIDTH: usize = 1024;
const SCREEN_HEIGHT: usize = 400;
const COLOR_BYTES: usize = 3;
const LINE_SIZE: usize = COLOR_BYTES * SCREEN_WIDTH;

const BAR_PXL_WIDTH: usize = 4;
const BAR_HEIGHT: usize = 50;
const BAR_SPACING: usize = 20;


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
                println!("Gamma:{}", *gamma);
            },
            _ => {}
        }
    }
    false
}

fn draw(renderer: &mut Renderer, texture: &mut Texture, gamma: f32){
    texture.with_lock(None, |buffer: &mut [u8], _| {

        let clines = buffer.chunks_mut(
            LINE_SIZE *(BAR_SPACING + BAR_HEIGHT));

        for cline in clines.zip(0..4) {
            for line in cline.0.chunks_mut(LINE_SIZE).enumerate() {
                let rep = Repeater::new(0..255, BAR_PXL_WIDTH);

                if line.0 <= BAR_SPACING {
                    for pxl in line.1.chunks_mut(COLOR_BYTES) {                        
                        let npxl: [u8; 3] = [0, 0, 0];
                        pxl.clone_from_slice(&npxl);
                    }
                } else if cline.1 == 0 {
                    for pxl in line.1.chunks_mut(COLOR_BYTES).zip(rep) {
                        let c = GammaRgb::new_u8(pxl.1, pxl.1, pxl.1, gamma).to_linear();
                        let npxl: [u8; COLOR_BYTES] = c.to_pixel();
                        pxl.0.clone_from_slice(&npxl);
                    }
                } else if cline.1 == 1 {
                    for pxl in line.1.chunks_mut(COLOR_BYTES).zip(rep) {
                        let c = GammaRgb::new_u8(pxl.1, 0, 0, gamma).to_linear();
                        let npxl: [u8; COLOR_BYTES] = c.to_pixel();
                        pxl.0.clone_from_slice(&npxl);
                    }
                } else if cline.1 == 2 {
                    for pxl in line.1.chunks_mut(COLOR_BYTES).zip(rep) {
                        let c = GammaRgb::new_u8(0, pxl.1, 0, gamma).to_linear();
                        let npxl: [u8; COLOR_BYTES] = c.to_pixel();
                        pxl.0.clone_from_slice(&npxl);
                    }
                } else if cline.1 == 3 {
                    for pxl in line.1.chunks_mut(COLOR_BYTES).zip(rep) {
                        let c = GammaRgb::new_u8(0, 0, pxl.1, gamma).to_linear();
                        let npxl: [u8; COLOR_BYTES] = c.to_pixel();
                        pxl.0.clone_from_slice(&npxl);
                    }
                }
            }
        }
    }).unwrap();
    renderer.copy(&texture, None, None);
    renderer.present();
}

fn main() {
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
