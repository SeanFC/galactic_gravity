let
  # Rolling updates, not deterministic.
  pkgs = import (fetchTarball("channel:nixpkgs-unstable")) {};
in pkgs.mkShell {
  buildInputs = [ 
    pkgs.cargo 
    pkgs.rustc 
    pkgs.SDL2 
    pkgs.emscripten 
  ];
  shellHook = ''
    rustup default stable
    rustup target add wasm32-unknown-emscripten

    git clone https://github.com/emscripten-core/emsdk.git
    cd emsdk
    ./emsdk install 1.39.20
    ./emsdk activate 1.39.20
    source ./emsdk_env.sh
    cd ..

    #cargo build -r --target wasm32-unknown-emscripten
  '';
}
