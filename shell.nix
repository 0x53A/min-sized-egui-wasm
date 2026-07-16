{ pkgs ? import <nixpkgs> {} }:

let
  nativeRuntimeLibs = with pkgs; [
    libGL
    libxkbcommon
    wayland
    libx11
    libxcursor
    libxi
    libxrandr
  ];
in

pkgs.mkShell {
  packages = with pkgs; [
    binaryen
    brotli
    curl
    gzip
    jq
    pkg-config
    trunk
    twiggy
    wasm-tools
    wabt
    wayland
    libxkbcommon
  ];

  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath nativeRuntimeLibs;

  shellHook = ''
    echo "wasm size tools: trunk, wasm-opt, wasm-tools, wasm2wat, twiggy, brotli, gzip"
    echo "native GUI libs: wayland, x11, libxkbcommon, libGL"
  '';
}
