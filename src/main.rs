extern crate sdl2;
extern crate gfx_backend_metal as back;
extern crate gfx_hal as hal;
extern crate winit;
extern crate rand;

pub mod game;

pub fn main() {

    game::game_loop();

}
