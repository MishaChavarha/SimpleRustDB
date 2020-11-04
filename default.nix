let
  rust_moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
in
with import <nixpkgs> {
  overlays = [ rust_moz_overlay ];
};
let
  rust_latest = latest.rustChannels.nightly.rust.override {
     extensions = ["rust-src"];
  };
in
mkShell {

  name = "rust-env";
  nativeBuildInputs = with buildPackages; [
    vscode # visual studop code

    # latest.rustChannels.nightly.rust.override {extensions = [ "rust-src" ];}
    # rustup
    pkg-config
    git
    rust_latest

    rust-analyzer
    rls
    lldb
  ];

  buildInputs = [];

  RUST_BACKTRACE = 1;

  shellHook = '''';
}
