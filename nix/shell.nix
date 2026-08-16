{
  perSystem = {
    pkgs,
    toolchain,
    ...
  }: {
    devShells.default = let
      packages = with pkgs; [
        git

        toolchain
        clang
        mold

        just

        watchexec
        sqlx-cli

        cargo-machete
        cargo-shear
        cargo-udeps
        cargo-features-manager

        cargo-audit
        cargo-deny
        cargo-outdated

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
        };
  };
}
