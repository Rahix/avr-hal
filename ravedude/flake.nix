{
  inputs = {
    utils = {
      url = "github:numtide/flake-utils";
    };

    naersk = {
      url = "github:nix-community/naersk";
    };
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk {};

        lib = pkgs.lib;

      in
      rec {
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
