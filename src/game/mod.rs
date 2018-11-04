use std::collections::hash_map::HashMap;
use std::time::{Duration, Instant};

use winit;
use winit::WindowEvent::*;
use back;

pub mod graphics;
pub mod units;

pub fn game_loop() {
    const FPS: u32 = 60;

    //winit
    let mut events_loop = winit::EventsLoop::new();
    let window = winit::WindowBuilder::new()
        .with_title("Your faithful window")
        .with_dimensions(winit::dpi::LogicalSize { width: 800.0, height: 600.0 })
        .build(&events_loop)
        .unwrap();
    //let mut texcache: HashMap<String, Texture> = HashMap::new();

    //add the block sprite to our cache
    let block = String::from("../../assets/block.BMP");
    //graphics::load_sprites(block, &texture_creator, &mut texcache);
    let background = String::from("../../assets/tet.BMP");
    //graphics::load_sprites(background, &texture_creator, &mut texcache);

    let mut starting_time: Instant = Instant::now();
    let mut pos = 40;

    let mut i = 0;
    //graphics::update(&mut canvas, &mut texcache, i as u8, pos);    
    events_loop.run_forever(|event| {

        match event {
            winit::Event::WindowEvent { event, .. } => match event {                
                winit::WindowEvent::KeyboardInput {
                    input:
                        winit::KeyboardInput {
                            virtual_keycode: Some(winit::VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } |
                CloseRequested => { return winit::ControlFlow::Break; },
                _ => (),
            },
            _ => (),
        }

        let loop_time: Instant = Instant::now();
        i = (i + 1) % 255;

        let ending_time: Duration = Instant::now().duration_since(loop_time);
        let delta_time: Duration = Duration::from_millis(250);

        if starting_time.elapsed() > delta_time {
            starting_time = Instant::now();
            if pos <= 466 {pos = pos + 26;}
            //graphics::update(&mut canvas, &mut texcache, i as u8, pos);
        }

        match (Duration::from_millis(1000) / FPS).checked_sub(ending_time) {
            Some(i) => ::std::thread::sleep(i),
            _ => (),
        };

        winit::ControlFlow::Continue
        
        //debug
        //println!("{:#?}", Instant::now().duration_since(starting_time));
    });
}
