extern crate sdl2;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::surface::Surface;
use sdl2::rect::Rect;
use std::path::Path;

//static png: &Path = Path::new("./assets/block.BMP");


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

    pub fn update(&mut self, i: u8) {
        let surface = match Surface::load_bmp(Path::new("../../assets/block.BMP")) {
            Ok(surface) => surface,
            Err(err) => panic!("failed to load: {}", err)
        };

        let tex_creator = self.canvas.texture_creator();

        let mut texture = match tex_creator.create_texture_from_surface(surface){
            Ok(texture) => texture,
            Err(err) => panic!("failed to set surface: {}", err)
        };

        let f = i as i32;
        self.canvas.set_draw_color(Color::RGB(i, 200, 255 - i));
        self.canvas.clear();
        self.canvas.copy(&texture, None, Some(Rect::new(100, f + 200, 32, 32))).unwrap();
        self.canvas.copy(&texture, None, Some(Rect::new(100, f + 232, 32, 32))).unwrap();
        self.canvas.copy(&texture, None, Some(Rect::new(100, f + 264, 32, 32))).unwrap();
        self.canvas.copy(&texture, None, Some(Rect::new(132, f + 200, 32, 32))).unwrap();
        
        
        /*self.canvas.copy_ex(&texture, None,
        Some(Rect::new(450, 100, 256, 256)), 30.0, None, false, false).unwrap();*/

        self.canvas.present();
    }
}


