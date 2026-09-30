{
  description = "Rust bindings for libnotify";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs =
    inputs@{ flake-parts, ... }:
    # https://flake.parts/module-arguments.html
    flake-parts.lib.mkFlake { inherit inputs; } ({
      flake = {
        # Put your original flake attributes here.
      };

      systems = [
        "x86_64-linux"
      ];

      perSystem = { config, pkgs, ... }: {
        # Recommended: move all package definitions here.
        # e.g. (assuming you have a nixpkgs input)
        # packages.foo = pkgs.callPackage ./foo/package.nix { };
        # packages.bar = pkgs.callPackage ./bar/package.nix {
        #   foo = config.packages.foo;
        # };
        packages.default = pkgs.callPackage ./package.nix { };
      };
    });
}
