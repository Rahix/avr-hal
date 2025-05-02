{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = { nixpkgs, utils, naersk, fenix, ... }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ fenix.overlays.default ];
        };
        lib = pkgs.lib;

        rust-toolchain = pkgs.fenix.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "X/4ZBHO3iW0fOenQ3foEvscgAPJYl2abspaBThDOukI=";
        };
        naersk' = pkgs.callPackage naersk {
          cargo = rust-toolchain;
          rustc = rust-toolchain;
        };
      in {
        packages.default = naersk'.buildPackage {
          pname = "ravedude";
          src = ./.;

          buildInputs = with pkgs; lib.optionals pkgs.stdenv.isLinux [
            pkg-config
            udev
          ];
        };
      }
    );
}
