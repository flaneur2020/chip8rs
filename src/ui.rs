
use sdl2;
use vm;

struct UI {
    canvas: sdl2::Canvas<sdl2::Window>,
}

const SCALE_FACTOR: u32 = 20;
const SCREEN_WIDTH: u32 = vm::CHIP8_WIDTH * SCALE_FACTOR;
const SCREEN_HEIGHT: u32 = vm::CHIP8_HEIGHT * SCALE_FACTOR;



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
        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
        UI { canvas: canvas }
    }

    pub fn draw(&mut self, pixels: &[[u8; CHIP8_WIDTH]; CHIP8_HEIGHT]) {
        for (y, row) in pixels.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                let x = (x as u32) * SCALE_FACTOR;
                let y = (y as u32) * SCALE_FACTOR;

                self.canvas.set_draw_color(self.get_color(col));
                self.canvas.fill_rect(Rect::new(x as i32, y as i32, SCALE_FACTOR, SCALE_FACTOR))
            }
        }
    }

    fn get_color(&mut self, v: u8) -> sdl2::pixels::Color {
        if v == 0 {
            sdl2::pixels::Color::RGB(0, 0, 0)
        } else {
            sdl2::pixels::Color::RGB(0, 250, 0)
        }
    }
}