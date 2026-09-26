{
  description = "Adriel Barrientos — portfolio, a Telar web-dom app";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # The CLI has to match the library `[patch.crates-io]` points at: an older transpiler writes `compile_error!` into `.telar/` for attributes it doesn't know. `git+file` because a path input would copy telar's multi-GB `target/`; pinned to the committed `dev` tip, so run `nix flake update telar` after committing in telar. Goes back to crates.io when telar is released.
    telar = {
      url = "git+file:///home/adrielgbm/projects/code/telar?ref=dev";
      flake = false;
    };
  };

  outputs =
    { nixpkgs, rust-overlay, telar, ... }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
    in
    {
      devShells = forAllSystems (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
          };
          rustToolchain = pkgs.rust-bin.stable.latest.default.override {
            targets = [ "wasm32-unknown-unknown" ];
            extensions = [ "rust-src" "rust-analyzer" "rustfmt" "clippy" ];
          };
          rustPlatform = pkgs.makeRustPlatform {
            cargo = rustToolchain;
            rustc = rustToolchain;
          };
          cargo-telar = rustPlatform.buildRustPackage {
            pname = "cargo-telar";
            version = (pkgs.lib.importTOML "${telar}/Cargo.toml").workspace.package.version;
            src = telar;
            cargoLock = {
              lockFile = "${telar}/Cargo.lock";
              outputHashes = {
                "base-db-0.0.0" = "sha256-cq+wrv+Zadl1NiJIWxYKHpcXfUQXDKBYa/7eq0JWFZk=";
              };
            };
            cargoBuildFlags = [ "-p" "cargo-telar" ];
            doCheck = false;
          };
        in
        {
          default = pkgs.mkShell {
            packages = [
              rustToolchain
              cargo-telar
              # Must match the wasm-bindgen crate version telar pins, or the generated glue is rejected.
              pkgs.wasm-bindgen-cli_0_2_127
              pkgs.binaryen
              pkgs.mold
            ];
            RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
            "CARGO_TARGET_${pkgs.stdenv.hostPlatform.rust.cargoEnvVarTarget}_RUSTFLAGS" = "-C link-arg=-fuse-ld=mold";
          };
        }
      );
    };
}
