# Galactic Gravity

A sandbox to play around with particles and planets acting under gravity. 
Also, a test bed to learn:
* Rust
* WebAssembly
* SDL2
* Github Pages

## Running

This project is managed by [Cargo](https://doc.rust-lang.org/cargo/), make sure it is installed in your environment.
Then, to run the project, use the command
```
cargo run
```
The project should respond to all the usual Cargo commands.

## Notes
* Good docs [here](http://web.archive.org/web/20201110143709/https://blog.therocode.net/2020/10/a-guide-to-rust-sdl2-emscripten)
* Pos need 
```
git clone https://github.com/emscripten-core/emsdk.git
cd emsdk
./emsdk install 1.39.20
./emsdk activate 1.39.20
source ./emsdk_env.sh
```
* Build with `cargo build --target=wasm32-unknown-emscripten`
* Run webserver with `python -m http.server -d target/wasm32--unknown-emscripten/debug`
* Cool rust wasm project [here](https://github.com/sandydoo/flux)
