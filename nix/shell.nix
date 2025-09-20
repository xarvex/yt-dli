{ pkgs, ... }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    cargo
    rustc

    clippy
    rust-analyzer
    rustfmt

    cargo-deny
    cargo-edit
    cargo-expand
    cargo-msrv
    cargo-release
    cargo-sort
    cargo-udeps

    deadnix
    flake-checker
    nixfmt-rfc-style
    statix

    just
    pre-commit
    (symlinkJoin {
      inherit (vhs)
        name
        pname
        version
        meta
        ;

      paths = [
        bashInteractive
        vhs
      ];
    })
  ];

  env = {
    RUST_BACKTRACE = 1;
    RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
  };

  shellHook = ''
    pre-commit install
  '';
}
