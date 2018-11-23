extern crate image;
extern crate rand;
extern crate gfx_hal as hal;
extern crate gfx_backend_metal as back;
extern crate winit;

pub mod renderer;
//pub mod teapot;
pub mod utils;

pub fn main() {
    renderer::render_loop();
}
