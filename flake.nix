{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    fenix,
    naersk,
  }:
    utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};

        toolchain = with fenix.packages.${system};
          combine [
            complete.rustc
            complete.rustfmt
            complete.cargo
            complete.clippy
            targets.wasm32-unknown-unknown.latest.rust-std
          ];
      in {
        defaultPackage =
          (naersk.lib.${system}.override {
            cargo = toolchain;
            rustc = toolchain;
          })
          .buildPackage {
            src = ./.;
            CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
          };
        devShell = with pkgs;
          mkShell {
            buildInputs = [
              toolchain
              rust-analyzer
              bacon
              trunk
              (pkgs.writeShellScriptBin "run" "npm run tauri dev")
            ];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
      }
    );
}
