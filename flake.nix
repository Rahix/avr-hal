{
  description = "AVR HAL development environment";

  inputs.devshell.url = "github:numtide/devshell";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.fenix.url = "github:nix-community/fenix";

  outputs =
    {
      self,
      fenix,
      flake-utils,
      devshell,
      nixpkgs,
    }:
    flake-utils.lib.eachDefaultSystem (system: {
      devShell =
        let
          pkgs = import nixpkgs {
            inherit system;

            overlays = [ devshell.overlays.default fenix.overlays.default ];
          };
        in
        pkgs.devshell.mkShell ({extraModulesPath, ...}: let
          rust-toolchain = pkgs.fenix.fromToolchainFile {
            file = ./rust-toolchain.toml;
            sha256 = "DnyK5MS+xYySA+csnnMogu2gtEfyiy10W0ATmAvmjGg=";
          };
          c-compiler = pkgs.pkgsCross.avr.buildPackages.gcc;
        in {
          name = "avr-hal";

          imports = [
            "${extraModulesPath}/language/c.nix"
          ];

          language.c = {
            compiler = c-compiler;
          };

          commands = [
            {
              name = "rustc";
              category = "rust";
              help = "Rust compiler";
              package = rust-toolchain;
            }
            {
              name = "cargo";
              category = "rust";
              help = "Rust build tool";
              package = rust-toolchain;
            }
            {
              name = "rustfmt";
              category = "rust";
              help = "Rust formatting tool";
              package = rust-toolchain;
            }
            {
              name = "avrdude";
              category = "avr";
              help = "Programmer for AVR chips";
              package = pkgs.avrdude;
            }
            {
              name = "ravedude";
              category = "avr";
              help = "Rust adapter for flashing with avrdude";
              package = pkgs.ravedude;
            }
            {
              name = "avr-gcc";
              category = "avr";
              help = "AVR C/C++ compiler (used for linking)";
              package = c-compiler;
            }
          ];
        });
    });
}
