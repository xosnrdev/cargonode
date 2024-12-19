{
  description = "Unified tooling for Node.js";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages = {
          default = pkgs.callPackage ./default.nix { };
        };

        devShells = {
          default = pkgs.callPackage ./shell.nix { };
        };

        formatter = pkgs.nixfmt-rfc-style;
      }
    );
}
