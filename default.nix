{ pkgs ? import <nixpkgs> { crossSystem.config = "armv7l-unknown-linux-gnueabihf"; } }:

with pkgs;
rustPlatform.buildRustPackage rec {
  pname = "rust-led-lights";
  version = "0.0.1";
  nativeBuildInputs = [
    binutils
    cargo
    rustc
    stdenv.cc
  ];

  src = ./.;

  cargoSha256 = "KRAbh4JWITiV9f7MwOmolm0EIiTCQiXSZvbSe9OCKnI="; # lib.fakeSha256;

}
