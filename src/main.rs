extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, Instant};

pub fn main() {
    const FPS: u32 = 60;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();


    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        let starting_time: Instant = Instant::now();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        canvas.present();

        let ending_time: Duration = Instant::now().duration_since(starting_time);

        match (Duration::from_millis(1000) / FPS).checked_sub(ending_time) {
            Some(i) => ::std::thread::sleep(i),
            _ => (),
        };

        //debug
        //println!("{:#?}", Instant::now().duration_since(starting_time));

    }
}
