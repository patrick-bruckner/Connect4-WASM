{
  description = "virtual environments";

  inputs.devshell.url = "github:numtide/devshell";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";
  inputs.android.url = "github:tadfisher/android-nixpkgs/stable";

  outputs = { self, flake-utils, devshell, nixpkgs, rust-overlay, android }:
    {
      overlay = final: prev: {
        rust-tauri-android = final.rust-bin.stable."1.70.0".minimal.override {
          targets = [
            "wasm32-unknown-unknown"
            "aarch64-linux-android"
            "arm-linux-androideabi"
            "armv7-linux-androideabi"
            "i686-linux-android"
            "thumbv7neon-linux-androideabi"
            "x86_64-linux-android"
          ];
        };
        inherit (self.packages.${final.system}) cargo-tauri-beta android-sdk;
      };
    } // flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            devshell.overlays.default
            (import rust-overlay)
            self.overlay
          ];
        };
      in
      {
        packages = {
          cargo-tauri-beta =
            with pkgs;
            stdenv.mkDerivation {
              name = "cargo-tauri-beta";
              buildInputs = [
                rust-bin.stable."1.70.0".minimal
                openssl
              ] ++ lib.optionals stdenv.isLinux [
                glibc
                libsoup
                cairo
                gtk3
                webkitgtk
              ] ++ lib.optionals stdenv.isDarwin [
                darwin.apple_sdk.frameworks.CoreServices
                darwin.apple_sdk.frameworks.Security
              ];
              ## don't unpack since sources are downloaded
              dontUnpack = true;
              installPhase = "mkdir -p $out; CARGO_HOME=$PWD cargo install tauri-cli --version \"^2.0.0-alpha\" --root $out";
            };
            android-sdk = android.sdk.${system} (sdkPkgs: with sdkPkgs; [
              build-tools-30-0-3
              cmdline-tools-latest
              emulator
              tools
              platform-tools
              platforms-android-33
              ndk-25-2-9519653
              patcher-v4
            ]);
        };
        devshell-config = pkgs.devshell.importTOML ./devshell.toml;
        devShell = pkgs.devshell.mkShell {
          imports = [
            self.devshell-config.${system}
          ];
        };
      }
    );
}
