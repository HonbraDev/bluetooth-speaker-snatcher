let
  rust_overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };
  rustVersion = "latest";
  rust = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
    extensions = [ "rust-src" ];
  });
in
pkgs.mkShell {
  buildInputs = [
    rust
  ] ++ (with pkgs; [
    rust-analyzer
    pkg-config
    dbus
  ]);
  RUST_BACKTRACE = 1;
}
