#nix-shell --command "python3 -m http.server -d target/wasm32-unknown-emscripten/debug"

nix-shell --command "cargo build --target wasm32-unknown-emscripten"

