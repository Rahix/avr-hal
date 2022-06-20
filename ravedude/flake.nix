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

      in
      rec {
        defaultPackage = naersk'.buildPackage {
          pname = "ravedude";
          src = ./.;

          buildInputs = with pkgs; [
            pkg-config
            udev
          ];
        };
      }
    );
}
