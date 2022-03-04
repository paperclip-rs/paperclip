{ profile ? "nightly", date ? "2022-02-22", oxalica ? "194016e6b086bfa5965aeb8979c58b93e03e2485" }:
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
