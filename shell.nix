{ pkgs ? import <nixpkgs> { }
,
}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt
    clippy
    rust-analyzer
    pkg-config
  ];

  shellHook = ''
    export RUST_BACKTRACE=1
    export RUST_LOG=debug
  '';
}
