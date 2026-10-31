{inputs, ...}: {
  imports = [
    ./shell.nix
  ];

  perSystem = {system, ...}: {
    _module.args = let
      pkgs = import inputs.nixpkgs {
        inherit system;
        overlays = [inputs.fenix.overlays.default];
      };

      toolchain = pkgs.fenix.fromToolchainFile {
        file = ../rust-toolchain.toml;
        sha256 = "sha256-07UyQu51OrDSHsy37ts4KC/NChHMfh5ZPY9QrihIXYo=";
        # sha256 = pkgs.lib.fakeHash;
      };
    in {
      inherit pkgs toolchain;
    };
  };
}
