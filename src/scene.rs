use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait SDLDrawable {
    fn draw(&self, canvas: &mut Canvas<Window>, particle_size: u32, colour: Color);
}

pub trait Tickable {
    fn push_forward(&mut self, time_delta: f64, particle_mass: f64);
}
