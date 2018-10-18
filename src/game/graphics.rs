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


pub fn load_sprites<'a>(location: String, texture_creator: &'a TextureCreator<WindowContext>, texcache: &mut HashMap<String, Texture<'a>>)/* -> (Texture<'a>*/{

    let path = Path::new(&location[..]);

    let surface = match Surface::load_bmp(&path) {
        Ok(surface) => surface,
        Err(err) => panic!("failed to load: {}", err)
    };

    match texcache.entry(location.clone()) {
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
    {
    let texture2 = match texcache.get_mut(&String::from("../../assets/tet.BMP")) {
        Some(texture2) => texture2,
        None => panic!("Failed to load block"),
    };

    canvas.clear();
    canvas.copy(&texture2, None, Some(Rect::new(0, 0, 800, 600))).unwrap();
    }

    let texture = match texcache.get_mut(&String::from("../../assets/block.BMP")) {
        Some(texture) => texture,
        None => panic!("Failed to load block"),
    };


    let f = i as i32;
    //canvas.set_draw_color(Color::RGB(i, 200, 255 - i));
    //canvas.clear();
    texture.set_color_mod(255, 0, 0);
    canvas.copy(&texture, None, Some(Rect::new(28, f + 200, 26, 26))).unwrap();
    canvas.copy(&texture, None, Some(Rect::new(28, f + 226, 26, 26))).unwrap();
    canvas.copy(&texture, None, Some(Rect::new(28, f + 252, 26, 26))).unwrap();
    canvas.copy(&texture, None, Some(Rect::new(54, f + 200, 26, 26))).unwrap();
    texture.set_color_mod(50, 255, 50);
    canvas.copy(&texture, None, Some(Rect::new(128, f + 200, 26, 26))).unwrap();
    canvas.copy(&texture, None, Some(Rect::new(154, f / 2 + 200, 26, 26))).unwrap();
    canvas.copy(&texture, None, Some(Rect::new(180, f + 200, 26, 26))).unwrap();
    canvas.copy(&texture, None, Some(Rect::new(128, f + 226, 26, 26))).unwrap();

    texture.set_color_mod(50, 50, 255);
    canvas.copy(&texture, None, Some(Rect::new(228, f + 200, 26, 26))).unwrap();
    canvas.copy(&texture, None, Some(Rect::new(228, f + 226, 26, 26))).unwrap();
    canvas.copy(&texture, None, Some(Rect::new(228, f + 252, 26, 26))).unwrap();
    canvas.copy(&texture, None, Some(Rect::new(254, f + 200, 26, 26))).unwrap();




    canvas.present();
}


