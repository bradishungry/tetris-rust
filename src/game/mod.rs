extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, Instant};

pub mod graphics;

pub fn game_loop() {

    const FPS: u32 = 60;

    let sdl_context = sdl2::init().unwrap();
    let mut graphics = graphics::Graphics::init(&sdl_context);
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

        graphics.update(i as u8);

        let ending_time: Duration = Instant::now().duration_since(starting_time);

        match (Duration::from_millis(1000) / FPS).checked_sub(ending_time) {
            Some(i) => ::std::thread::sleep(i),
            _ => (),
        };

        //debug
        //println!("{:#?}", Instant::now().duration_since(starting_time));

    }
}
