{ profile ? "nightly", date ? "2021-11-22" }:
let
  oxalica = builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/a1b1977429de5d69a332dd87700ffb00525335f9.tar.gz";
  pkgs = import <nixpkgs> {
    overlays = [ (import oxalica) ];
  };
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    rust-bin.${profile}.${date}.default
    openssl
    pkg-config
  ];
}
