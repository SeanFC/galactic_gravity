nix-shell --command "cargo build -r --target wasm32-unknown-emscripten"
mkdir -p build
cp web/* build/
cp target/wasm32-unknown-emscripten/release/galactic_gravity.js build/
cp target/wasm32-unknown-emscripten/release/galactic_gravity.wasm build/
