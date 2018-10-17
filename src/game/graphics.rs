extern crate sdl2;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl2::pixels::Color;
use sdl2::surface::Surface;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::rect::Rect;
use std::collections::hash_map::HashMap;
use std::collections::hash_map::Entry;
use std::path::Path;


pub fn load_sprites<'a>(texture_creator: &'a TextureCreator<WindowContext>, texcache: &mut HashMap<String, Texture<'a>>)/* -> (Texture<'a>*/{

    let path = Path::new("../../assets/block.BMP");

    let surface = match Surface::load_bmp(path) {
        Ok(surface) => surface,
        Err(err) => panic!("failed to load: {}", err)
    };

    match texcache.entry(String::from("block")) {
        Entry::Vacant(entry) => {
            match texture_creator.create_texture_from_surface(&surface) {
                Ok(texture) => {
                    entry.insert(texture);
                },
                Err(msg) => panic!("sprite could not be rendered: {:?}", msg)
            }
        },

        _ => {},
    };

}

pub fn update(canvas: &mut Canvas<Window>, texcache: &mut HashMap<String, Texture>, i: u8) {

    let texture = match texcache.get_mut(&String::from("block")) {
        Some(texture) => texture,
        None => panic!("Failed to load block"),
    };

    let f = i as i32;
    canvas.set_draw_color(Color::RGB(i, 200, 255 - i));
    canvas.clear();
    texture.set_color_mod(255, 0, 0);
    canvas.copy(&texture, None, Some(Rect::new(100, f + 200, 32, 32))).unwrap();
    canvas.copy(&texture, None, Some(Rect::new(100, f + 232, 32, 32))).unwrap();
    canvas.copy(&texture, None, Some(Rect::new(100, f + 264, 32, 32))).unwrap();
    canvas.copy(&texture, None, Some(Rect::new(132, f + 200, 32, 32))).unwrap();
    texture.set_color_mod(50, 255, 50);
    canvas.copy(&texture, None, Some(Rect::new(200, f + 200, 32, 32))).unwrap();
    canvas.copy(&texture, None, Some(Rect::new(232, f / 2 + 200, 32, 32))).unwrap();
    canvas.copy(&texture, None, Some(Rect::new(264, f + 200, 32, 32))).unwrap();
    canvas.copy(&texture, None, Some(Rect::new(200, f + 232, 32, 32))).unwrap();

    texture.set_color_mod(50, 50, 255);
    canvas.copy(&texture, None, Some(Rect::new(300, f + 200, 32, 32))).unwrap();
    canvas.copy(&texture, None, Some(Rect::new(300, f + 232, 32, 32))).unwrap();
    canvas.copy(&texture, None, Some(Rect::new(300, f + 264, 32, 32))).unwrap();
    canvas.copy(&texture, None, Some(Rect::new(332, f + 200, 32, 32))).unwrap();

    canvas.present();
}


