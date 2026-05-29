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
        sha256 = "sha256-rNsOYVHiSXXSDRGdg/StkiKCsyCTEPBfsP3R9spCu1c=";
      };
    in {
      inherit pkgs toolchain;
    };
  };
}
