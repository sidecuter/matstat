{
  description = "Flake for developing matstat solver";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system: 
    let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ (import rust-overlay) ];
      };
      rustToolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
        extensions = [ "rust-src" "clippy-preview" ];
        targets = [ 
          "wasm32-unknown-unknown"
        ];
      });
    in
    {
      devShells.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          trunk
          rustToolchain
          nodejs
          tailwindcss
          leptosfmt
          (writeShellScriptBin "clippy" ''
            cargo-clippy
          '')
        ];
      };
    });
}
