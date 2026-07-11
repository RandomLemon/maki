{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ rust-overlay.overlays.default ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in
      {
        devShell = pkgs.mkShell {
          packages = with pkgs; [
            rust
            pkg-config
            openssl.dev # 很多 crate 用到，提前塞进来
            clang # 当 linker，避 NixOS glibc 问题
          ];
          # 让 rustc 用 clang 而不是系统的 gcc（NixOS 上更稳）
          shellHook = ''
            export CC=clang
            export CXX=clang++
          '';
        };
      }
    );
}
