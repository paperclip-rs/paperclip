{ profile ? "nightly", date ? "2022-05-06", oxalica ? "43f4c4319fd29d07912a65d405ff03069c7748c4", rustup ? true }:
let
  oxalica_overlay = builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/${oxalica}.tar.gz";
  pkgs = import <nixpkgs> {
    overlays = [ (import oxalica_overlay) ];
  };
  rust = (pkgs.rust-bin.${profile}.${date}.default.override { extensions = [ "rust-src" ]; });
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    openssl
    pkg-config
  ] ++ pkgs.lib.optional (!rustup) rust;
  shellHook = ''
    cat <<EOF >rust-toolchain.toml
  [toolchain]
  channel = "${profile}-${date}"
  components = [ "rust-src" ]
  EOF
  '';
}
