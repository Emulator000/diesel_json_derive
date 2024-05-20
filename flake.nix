{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            cargo
            clang
            clippy
            rustc
          ];

          packages = with pkgs; [
            cargo-expand
            cargo-readme
            diesel-cli
            postgresql
            postgresql.lib
            rust-analyzer
            rustfmt
          ];

          RUST_BACKTRACE = "1";
          RUST_LOG = "debug";

          DATABASE_URL = "postgres://postgres@localhost:5433";
        };
      }
    );
}
