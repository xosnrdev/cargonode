{
  description =
    "A unified CLI tool that brings Cargo's developer experience to Node.js";

  inputs = {
    nixpkgs.url =
      "github:NixOS/nixpkgs?rev=de1864217bfa9b5845f465e771e0ecb48b30e02d";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        manifest = pkgs.lib.importTOML ./Cargo.toml;
        package = manifest.package;
        rustApp = pkgs.rustPlatform.buildRustPackage {
          pname = package.name;
          version = package.version;
          src = pkgs.lib.cleanSource ./.;
          cargoLock.lockFile = ./Cargo.lock;
          meta = with pkgs.lib; {
            inherit (package) description homepage repository;
            license = licenses.mit;
            maintainers = [ maintainers.xosnrdev ];
          };
        };

        devShell = pkgs.mkShell {
          buildInputs = [
            pkgs.cargo-watch
            pkgs.cargo-sort
            pkgs.git-cliff
            pkgs.cargo-release
            pkgs.cargo-edit
            pkgs.cargo-dist
            pkgs.cargo-tarpaulin
          ];
          shellHook = ''
            export RUST_BACKTRACE=1
          '';
        };

      in {
        formatter = pkgs.nixfmt-classic;
        packages = { default = rustApp; };
        devShells.default = devShell;
      });
}
