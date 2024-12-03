{
  description = "Unified tooling for Node.js";
  
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system: {
      packages = {
        default = nixpkgs.legacyPackages.${system}.callPackage ./default.nix { };
      };
      
      devShells = {
        default = nixpkgs.legacyPackages.${system}.callPackage ./shell.nix { };
        buildInputs = with nixpkgs.legacyPackages.${system}; [ bzip2 ];
        nativeBuildInputs = with nixpkgs.legacyPackages.${system}; [ pkg-config ];
      };
    });
}