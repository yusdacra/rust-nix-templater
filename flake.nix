{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    nci = {
      url = "github:yusdacra/nix-cargo-integration";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flakeCompat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = inputs:
    inputs.nci.lib.makeOutputs {
      root = ./.;
      systems = [
        "aarch64-linux"
        "aarch64-darwin"
        "i686-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];
      config = common: {
        outputs.defaults = {
          app = "rust-nix-templater";
          package = "rust-nix-templater";
        };
        shell = {
          packages = [common.pkgs.treefmt];
        };
      };
      pkgConfig = common: {
        rust-nix-templater = {
          build = true;
          app = true;
          overrides = {
            add-envs = {
              TEMPLATER_FMT_BIN = "${common.pkgs.alejandra}/bin/alejandra";
              TEMPLATER_CARGO_BIN = "${common.rustToolchain.cargo}/bin/cargo";
              NCI_SRC = toString inputs.nci;
            };
          };
        };
      };
    };
}
