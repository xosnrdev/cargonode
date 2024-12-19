{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  # Get dependencies from the main package
  inputsFrom = [ (pkgs.callPackage ./default.nix { }) ];
  # Additional tooling
  buildInputs = with pkgs; [
    bzip2
    nixfmt-rfc-style
  ];
  nativeBuildInputs = with pkgs; [ pkg-config ];
}
