{
  description = "A simple build tool for Node.js projects.";

  inputs = {
    nixpkgs.url =
      "github:NixOS/nixpkgs?rev=a47b881e04af1dd6d414618846407b2d6c759380";
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
            pkgs.git
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
