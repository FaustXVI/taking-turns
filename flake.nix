{
  description = "Torov";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }@inputs:
    (flake-utils.lib.eachDefaultSystem (system:
      let
         overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
          config.allowUnfree = true;
        };
        rustBuildInput = pkgs.rust-bin.stable."1.83.0".default.override{
          extensions = [ "rust-src" "rust-analyzer" "clippy" ];
          targets = [ "x86_64-unknown-linux-gnu" "wasm32-unknown-unknown" ];
        };
      in
      {
        devShells.default = pkgs.mkShell (
          {
            RUST_BACKTRACE = "full";
            #https://github.com/luakit/luakit/issues/1102
            WEBKIT_DISABLE_DMABUF_RENDERER=1;
            #https://docs.rs/getrandom/latest/getrandom/#webassembly-support
            RUSTFLAGS=''--cfg getrandom_backend="wasm_js"'';
            packages = with pkgs; [
              wasm-bindgen-cli_0_2_100
              dioxus-cli
              rustBuildInput
              pkg-config
              gtk3
              webkitgtk_4_1
              openssl
              libiconv
              glib
              libsoup_3
              xdotool
            ];
          }
        );
      }));
}
