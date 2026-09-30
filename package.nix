{
  pkgs,
  lib,
}:

let
  cargo = (fromTOML (builtins.readFile ./Cargo.toml));
in

pkgs.rustPlatform.buildRustPackage {
  pname = cargo.package.name;
  version = cargo.package.version;

  src = lib.cleanSource ./.;

  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = with pkgs; [
    clang
    pkg-config
    rustPlatform.bindgenHook
  ];

  buildInputs = with pkgs; [
    libnotify.dev
  ];
}
