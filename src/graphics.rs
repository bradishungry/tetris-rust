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

enum Shape {
    IBLOCK,
    LBLOCK,
    RLBLOCK,
    ZBLOCK,
    RZBLOCK,
    TBLOCK,
    SQBLOCK,
}

struct Blocks {
    color: (u8, u8, u8),
    block_pos: (i32, i32, i32, i32, i32, i32, i32, i32),
}


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

pub fn update(canvas: &mut Canvas<Window>, texcache: &mut HashMap<String, Texture>, i: u8, pos: i32) {
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

    let block_shape = Shape::ZBLOCK;
    match block_shape {
        Shape::RLBLOCK => { let block = Blocks { color: (255, 100, 0), block_pos: (28, 28, 28, 54, pos, pos + 26, pos + 52, pos + 52) }; 
                            blit_block(block, canvas, texture);},

        Shape::SQBLOCK => { let block = Blocks { color: (255, 255, 50), block_pos: (28, 54, 28, 54, pos, pos, pos + 26, pos + 26) }; 
                            blit_block(block, canvas, texture);},

        Shape::LBLOCK => { let block = Blocks { color: (0, 0, 255), block_pos: (28, 54, 54, 54, pos + 52, pos + 52, pos + 26, pos) }; 
                            blit_block(block, canvas, texture);},

        Shape::ZBLOCK => { let block = Blocks { color: (255, 0, 0), block_pos: (28, 54, 54, 80, pos + 26, pos + 26, pos, pos) }; 
                            blit_block(block, canvas, texture);},

        Shape::RZBLOCK => { let block = Blocks { color: (0, 255, 0), block_pos: (28, 54, 54, 80, pos, pos, pos + 26, pos + 26) }; 
                            blit_block(block, canvas, texture);},

        Shape::TBLOCK => { let block = Blocks { color: (255, 0, 255), block_pos: (28, 54, 54, 80, pos + 26, pos + 26, pos, pos + 26) }; 
                            blit_block(block, canvas, texture);},

        Shape::IBLOCK => { let block = Blocks { color: (0, 255, 255), block_pos: (28, 28, 28, 28, pos, pos + 26, pos + 52, pos + 78) };
                            blit_block(block, canvas, texture);},

        //_ => println!("NAH"),
    };


    let f = i as i32;
    //canvas.set_draw_color(Color::RGB(i, 200, 255 - i));
    //canvas.clear();
        /*texture.set_color_mod(50, 255, 50);
    canvas.copy(&texture, None, Some(Rect::new(28 + 2*26, pos + 26, 26, 26))).unwrap();
    canvas.copy(&texture, None, Some(Rect::new(28 + 3*26, pos + 52, 26, 26))).unwrap();
    canvas.copy(&texture, None, Some(Rect::new(28 + 4*26, pos + 52, 26, 26))).unwrap();
    canvas.copy(&texture, None, Some(Rect::new(28 + 2*26, pos + 52, 26, 26))).unwrap();

    texture.set_color_mod(50, 50, 255);
    canvas.copy(&texture, None, Some(Rect::new(28 + 5*26, pos, 26, 26))).unwrap();
    canvas.copy(&texture, None, Some(Rect::new(28 + 5*26, pos + 26, 26, 26))).unwrap();
    canvas.copy(&texture, None, Some(Rect::new(28 + 5*26, pos + 52, 26, 26))).unwrap();
    canvas.copy(&texture, None, Some(Rect::new(28 + 6*26, pos + 52, 26, 26))).unwrap();

    texture.set_color_mod(255, 255, 50);
    canvas.copy(&texture, None, Some(Rect::new(28 + 7*26, pos + 26, 26, 26))).unwrap();
    canvas.copy(&texture, None, Some(Rect::new(28 + 8*26, pos + 52, 26, 26))).unwrap();
    canvas.copy(&texture, None, Some(Rect::new(28 + 9*26, pos + 52, 26, 26))).unwrap();
    canvas.copy(&texture, None, Some(Rect::new(28 + 7*26, pos + 52, 26, 26))).unwrap();*/

    texture.set_color_mod(255, 255, 50);
    canvas.copy(&texture, None, Some(Rect::new(313, 40 + 13, 13, 13))).unwrap();
    canvas.copy(&texture, None, Some(Rect::new(313 + 13, 40 + 13, 13, 13))).unwrap();
    canvas.copy(&texture, None, Some(Rect::new(313 + 2*13, 40 + 13, 13, 13))).unwrap();
    canvas.copy(&texture, None, Some(Rect::new(313, 40 + 26, 13, 13))).unwrap();




    canvas.present();
}

fn blit_block(block: Blocks, canvas: &mut Canvas<Window>, texture: &mut Texture){
    texture.set_color_mod(block.color.0, block.color.1, block.color.2);
    canvas.copy(&texture, None, Some(Rect::new(block.block_pos.0, block.block_pos.4, 26, 26))).unwrap();
    canvas.copy(&texture, None, Some(Rect::new(block.block_pos.1, block.block_pos.5, 26, 26))).unwrap();
    canvas.copy(&texture, None, Some(Rect::new(block.block_pos.2, block.block_pos.6, 26, 26))).unwrap();
    canvas.copy(&texture, None, Some(Rect::new(block.block_pos.3, block.block_pos.7, 26, 26))).unwrap();
}


