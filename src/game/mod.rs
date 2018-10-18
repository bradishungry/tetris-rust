extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::hash_map::HashMap;
use sdl2::render::Texture;
use sdl2::pixels::Color;
use std::time::{Duration, Instant};

pub mod graphics;

pub fn game_loop() {

    const FPS: u32 = 60;

    //init SDL rendering things
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("tetris", 800, 600).position_centered().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texcache: HashMap<String, Texture> = HashMap::new();

    //add the block sprite to our cache
    let block = String::from("../../assets/block.BMP");
    graphics::load_sprites(block, &texture_creator, &mut texcache);
    let background = String::from("../../assets/tet.BMP");
    graphics::load_sprites(background, &texture_creator, &mut texcache);

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut i = 0;
    'running: loop {
        let starting_time: Instant = Instant::now();
        i = (i + 1) % 255;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        graphics::update(&mut canvas, &mut texcache, i as u8);

        let ending_time: Duration = Instant::now().duration_since(starting_time);

        match (Duration::from_millis(1000) / FPS).checked_sub(ending_time) {
            Some(i) => ::std::thread::sleep(i),
            _ => (),
        };

        //debug
        //println!("{:#?}", Instant::now().duration_since(starting_time));

    }
}
