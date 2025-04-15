{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    # Rust toolchain
    rustc
    cargo
    rustfmt
    clippy

    # SDL2 libraries
    SDL2
    SDL2_image
    SDL2_ttf

    # Build essentials
    pkg-config
  ];

  # Set environment variables to help the linker find libraries
  shellHook = ''
    export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [
      pkgs.SDL2
      pkgs.SDL2_image
      pkgs.SDL2_ttf
    ]}:$LD_LIBRARY_PATH"
  '';
}
