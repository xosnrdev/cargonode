{
  description = "Unified tooling for Node.js";

  # ----------------------------------------------------------------------------
  # Inputs
  # ----------------------------------------------------------------------------
  inputs = {
    # Using a pinned nixpkgs reference for consistency
    nixpkgs.url =
      "github:NixOS/nixpkgs?rev=de1864217bfa9b5845f465e771e0ecb48b30e02d";
    flake-utils.url = "github:numtide/flake-utils";
  };

  # ----------------------------------------------------------------------------
  # Outputs
  # ----------------------------------------------------------------------------
  outputs = { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        manifest = pkgs.lib.importTOML ./Cargo.toml;
        package = manifest.package;

        # ----------------------------------------------------------------------
        # Rust Build
        # ----------------------------------------------------------------------
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

        # ----------------------------------------------------------------------
        # Development Shell
        # ----------------------------------------------------------------------
        devShell = pkgs.mkShell {
          buildInputs = [
            # Rust & dev tooling
            pkgs.cargo-watch
            pkgs.cargo-sort
            pkgs.git-cliff
            pkgs.cargo-release
            pkgs.cargo-edit
          ];

          shellHook = ''
            export RUST_BACKTRACE=1
          '';
        };

      in {
        # Formatter
        formatter = pkgs.nixfmt-classic;

        # Packages
        packages = { default = rustApp; };

        # Development Shell
        devShells.default = devShell;
      });
}
