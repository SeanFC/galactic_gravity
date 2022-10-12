# Galactic Gravity

[Live demo](https://seanfc.github.io/galactic_gravity/)

A sandbox to play around with particles and planets acting under gravity. 

## Run - Desktop

The rust part of the project is managed by [Cargo](https://doc.rust-lang.org/cargo/), make sure it is installed in your environment.
Then, to run the project, use the command
```
cargo run
```
The project should respond to all the usual Cargo commands.

## Run - Web
### Build

To build the project for the web run
```
./scripts/build.sh
```
which will place the results in `build/`.
Running a server targeting this directory will server the project to a web page. 
If `python` is available this can be done by running
```
python -m http.server -d build
```
and visiting the suggest location, likely [https://0.0.0.0:8000](https://0.0.0.0:8000)

### Nix

The build environment for the project for the web is handled by [Nix](https://nixos.org/). 
Make sure you have Nix installed and run
```
nix-shell --command "cargo build -r --target wasm32-unknown-emscripten"
```
