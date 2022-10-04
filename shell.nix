{ profile ? "nightly", date ? "2022-09-19", oxalica ? "5db6b63124ccedd61e896ec98def85fb4e6668f4", rustup ? true }:
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
