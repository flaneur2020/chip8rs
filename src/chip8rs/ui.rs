
use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use super::vm::{CHIP8_HEIGHT, CHIP8_WIDTH};

pub struct UI {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    events: sdl2::EventPump,
}

const SCALE_FACTOR: u32 = 5;
const SCREEN_WIDTH: u32 = CHIP8_WIDTH as u32 * SCALE_FACTOR;
const SCREEN_HEIGHT: u32 = CHIP8_HEIGHT as u32 * SCALE_FACTOR;

//
// Keypad                   Keyboard
// +-+-+-+-+                +-+-+-+-+
// |1|2|3|C|                |1|2|3|4|
// +-+-+-+-+                +-+-+-+-+
// |4|5|6|D|                |Q|W|E|R|
// +-+-+-+-+       =>       +-+-+-+-+
// |7|8|9|E|                |A|S|D|F|
// +-+-+-+-+                +-+-+-+-+
// |A|0|B|F|                |Z|X|C|V|
// +-+-+-+-+                +-+-+-+-+
//


impl UI {
    pub fn new(sdl_context: sdl2::Sdl) -> UI {
        let video = sdl_context.video().unwrap();
        let window = video
            .window("sdl2", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
        UI {
            canvas: canvas,
            events: sdl_context.event_pump().unwrap(),
        }
    }

    pub fn draw(&mut self, pixels: &[[u8; CHIP8_WIDTH]; CHIP8_HEIGHT]) {
        for (y, row) in pixels.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = (x as u32) * SCALE_FACTOR;
                let y = (y as u32) * SCALE_FACTOR;

                let c = self.get_color(col);
                self.canvas.set_draw_color(c);
                self.canvas.fill_rect(Rect::new(x as i32, y as i32, SCALE_FACTOR, SCALE_FACTOR));
            }
        }
        self.canvas.present();
    }

    pub fn poll(&mut self) -> Result<[bool; 16], ()> {
        let mut chip8_keys = [false; 16];

        for event in self.events.poll_iter() {
            if let Event::Quit { .. } = event {
                return Err(());
            }
        }

        let keys: Vec<Keycode> = self.events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();
        
        for key in keys {
            let index = match key {
                Keycode::Num1 => Some(0x1),
                Keycode::Num2 => Some(0x2),
                Keycode::Num3 => Some(0x3),
                Keycode::Num4 => Some(0xc),
                Keycode::Q => Some(0x4),
                Keycode::W => Some(0x5),
                Keycode::E => Some(0x6),
                Keycode::R => Some(0xd),
                Keycode::A => Some(0x7),
                Keycode::S => Some(0x8),
                Keycode::D => Some(0x9),
                Keycode::F => Some(0xe),
                Keycode::Z => Some(0xa),
                Keycode::X => Some(0x0),
                Keycode::C => Some(0xb),
                Keycode::V => Some(0xf),
                _ => None,
            };

            if let Some(i) = index {
                chip8_keys[i] = true;
            }
        }
        Ok(chip8_keys)
    }

    fn get_color(&mut self, v: u8) -> Color {
        if v == 0 {
            Color::RGB(0, 0, 0)
        } else {
            Color::RGB(0, 250, 0)
        }
    }
}