{
  description = "yt-dli";

  inputs = {
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };

    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    systems.url = "github:nix-systems/default-linux";
  };

  outputs =
    inputs@{
      flake-parts,
      nixpkgs,
      self,
      ...
    }:
    let
      inherit (nixpkgs) lib;
    in
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;

      perSystem =
        { pkgs, ... }:
        {
          packages = rec {
            default = yt-dli;
            yt-dli = pkgs.callPackage ./nix/package.nix { };
          };

          devShells = rec {
            default = yt-dli;
            yt-dli = import ./nix/shell.nix {
              inherit
                inputs
                lib
                pkgs
                self
                ;
            };
          };

          formatter = pkgs.nixfmt-rfc-style;
        };

      flake.homeModules = rec {
        default = yt-dli;
        yt-dli = import ./nix/home.nix { inherit inputs self; };
      };
    };
}
