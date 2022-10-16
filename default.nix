let
  arm = import <nixpkgs> { crossSystem.config = "armv7l-unknown-linux-musleabihf"; };
  rust_overlay = import (builtins.fetchTarball https://github.com/oxalica/rust-overlay/archive/master.tar.gz);
  unstable = import <nixos-unstable-small> { };
  nixpkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };
  myrust = nixpkgs.rust-bin.stable."1.63.0".default.override {
    extensions = [ "rust-src" ];
    targets = [ "armv7-unknown-linux-musleabihf" ];
  };

in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "rust-shell";
    buildInputs = [
      arm.stdenv.cc
      unstable.rust-analyzer  # must be before rust so the PATH picks this one first
      myrust
    ];
    RUST_SRC_PATH = "${myrust}/lib/rustlib/src/rust/src";
    LD_LIBRARY_PATH = "${arm.binutils-unwrapped}/lib";
  }