{
  description = "rust flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default =
          with pkgs;
          mkShell {
            buildInputs = [
              # Rust dependencies
              (rust-bin.stable.latest.default.override {
                extensions = [
                  "rust-src"
                  "rust-analyzer"
                  "clippy"
                ];
              })
              pkg-config
              pkgs.lldb

              pkgs.just

              pkgs.sea-orm-cli
            ];
            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
            RUST_BACKTRACE = 1;
          };

        packages = rec {
          app = pkgs.rustPlatform.buildRustPackage {
            pname = "bewerbungs_tool";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
          };

          dockerImage = pkgs.dockerTools.buildImage {
            name = "bewerbungs_tool";
            tag = "latest";
            copyToRoot = pkgs.buildEnv {
              name = "image-root";
              paths = [ app ];
              pathsToLink = [ "/bin" ];
            };
            config = {
              Cmd = [ "/bin/bewerbungs_tool" ];
              WorkingDir = "/";
            };
          };

          default = dockerImage;
        };
      }
    );
}
