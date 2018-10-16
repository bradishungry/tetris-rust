extern crate sdl2;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;

pub struct Graphics{
    //pub video_subsystem: VideoSubsystem,
    //pub window: Window,
    pub canvas: Canvas<Window>,
}

impl Graphics {
    pub fn init(sdl_context: &sdl2::Sdl) -> Self {
        let v = sdl_context.video().unwrap();
        let w = v.window("tetris", 800, 600)
            .position_centered()
            .build()
            .unwrap();
        let mut c = w.into_canvas().build().unwrap();

        c.set_draw_color(Color::RGB(0, 255, 255));
        c.clear();
        c.present();

        Graphics { canvas: c }
    }

    pub fn update(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 200, 255));
        self.canvas.clear();
        self.canvas.present();
    }
}


