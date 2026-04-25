{
  pkgs ?
    import <nixpkgs> {
      overlays = [
        (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
      ];
    },
}: let
  packages = with pkgs; [
    (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)

    just

    watchexec
    sqlx-cli
    cargo-udeps
    cargo-audit
    cargo-expand
  ];

  libraries = with pkgs; [
    pkg-config
    openssl
  ];
in
  with pkgs;
    mkShell {
      name = "rust-backend-template";
      buildInputs = packages ++ libraries;
      hardeningDisable = ["fortify"];

      DIRENV_LOG_FORMAT = "";
      LD_LIBRARY_PATH = "${lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH";
    }
