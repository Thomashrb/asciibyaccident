with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "rust-env";
  buildInputs = [
    rustracer
    rustPlatform.rustcSrc
    rustc
    rustfmt
    cargo
  ];

  shellHook = ''
    echo 'Entering Rust Project Environment'
  '';

  # Set Environment Variables
  # RUST_BACKTRACE = 1;
  RUST_SRC_PATH = "${rustPlatform.rustcSrc}";
}
