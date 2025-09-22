{
  description = "Torov";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    nix-filter.url = "github:numtide/nix-filter";

    crane.url = "github:ipetkov/crane";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, nix-filter, crane, advisory-db, ... }@inputs:
    (flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain (p: p.rust-bin.stable."1.86.0".default.override {
          extensions = [ "rust-src" "llvm-tools-preview" "clippy" "rustfmt" "cargo" ];
          targets = [ "x86_64-unknown-linux-gnu" "wasm32-unknown-unknown" ];
        });

        unfilteredRoot = ./.;
        src = pkgs.lib.fileset.toSource {
          root = unfilteredRoot;
          fileset = pkgs.lib.fileset.unions [
            ./LICENSE
            ./assets
            # Default files from crane (Rust and cargo files)
            (craneLib.fileset.commonCargoSources unfilteredRoot)
            # Extra resources
            (pkgs.lib.fileset.maybeMissing ./testing)
          ];
        };
        commonArgs = {
          inherit src;
          strictDeps = true;
          buildInputs = with pkgs; [
            wasm-bindgen-cli_0_2_100
            dioxus-cli
            pkg-config
            gtk3
            webkitgtk_4_1
            openssl
            libiconv
            glib
            libsoup_3
            xdotool
          ];
        };

        # Build crate dependencies independently in order to speed up this crate build.
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      in
      {
        checks = import ./nix/checks.nix { inherit self system pkgs craneLib cargoArtifacts commonArgs src advisory-db; };
        devShells = {
          default = craneLib.devShell (commonArgs // {
            packages = with pkgs; [
              taplo
              cargo-audit
              cargo-deny
            ] ++ commonArgs.buildInputs;
            RUST_BACKTRACE = "full";
            #https://github.com/luakit/luakit/issues/1102
            WEBKIT_DISABLE_DMABUF_RENDERER = 1;
            #https://docs.rs/getrandom/latest/getrandom/#webassembly-support
            RUSTFLAGS = ''--cfg getrandom_backend="wasm_js"'';
          });
        };
      }));
}
