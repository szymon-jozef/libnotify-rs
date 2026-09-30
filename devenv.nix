{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

{
  # https://devenv.sh/packages/
  packages = with pkgs; [
    git

    clang

    libnotify.dev

    pkg-config
    rustPlatform.bindgenHook
  ];

  # https://devenv.sh/languages/
  languages.rust.enable = true;

}
