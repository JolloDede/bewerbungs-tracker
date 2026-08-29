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
                targets = [
                ];
              })
              pkg-config
              pkgs.lldb

              pkgs.just

              pkgs.sea-orm-cli
             # Test
              pkgs.cacert
              pkgs.curl
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

          # Image for x86-64
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

          appAarch64 = pkgs.pkgsCross.aarch64-multiplatform.rustPlatform.buildRustPackage {
            pname = "bewerbungs_tool";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
          };

          # Image for the Raspberrypi
          dockerImageAarch64 = pkgs.dockerTools.buildImage {
            name = "bewerbungs_tool";
            tag = "arm64-v8";
            copyToRoot = pkgs.buildEnv {
              name = "image-root";
              paths = [ appAarch64 ];
              pathsToLink = [ "/bin" ];
            };
            config = {
              Cmd = [ "/bin/bewerbungs_tool" ];
              WorkingDir = "/";
            };
          };

          aarch64 =
            let
              staticPkgs = pkgs.pkgsCross.aarch64-multiplatform.pkgsStatic;
              rustAarch64 = pkgs.rust-bin.stable.latest.default.override {
                targets = [ "aarch64-unknown-linux-musl" ];
              };
            in
            staticPkgs.rustPlatform.buildRustPackage {
              pname = "bewerbungs_tool";
              version = "0.1.0";
              src = ./.;
              cargoLock.lockFile = ./Cargo.lock;
              cargo = rustAarch64;
              rustc = rustAarch64;

              postInstall = ''
                cp -r api/assets $out/bin/assets
              '';
            };

          default = dockerImage;
        };
      }
    );
}
