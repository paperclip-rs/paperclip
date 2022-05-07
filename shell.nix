{ profile ? "nightly", date ? "2022-05-06", oxalica ? "43f4c4319fd29d07912a65d405ff03069c7748c4" }:
let
  oxalica_overlay = builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/${oxalica}.tar.gz";
  pkgs = import <nixpkgs> {
    overlays = [ (import oxalica_overlay) ];
  };
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    rust-bin.${profile}.${date}.default
    openssl
    pkg-config
  ];
}
