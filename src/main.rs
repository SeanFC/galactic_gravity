extern crate emscripten_main_loop;

pub mod game;

/// Main program loop
pub fn main() {
    let game = game::Game::new().unwrap();
    emscripten_main_loop::run(game);
}
