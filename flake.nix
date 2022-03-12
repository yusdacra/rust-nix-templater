{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    nixCargoIntegration = {
      url = "github:yusdacra/nix-cargo-integration";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flakeCompat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = inputs:
    inputs.nixCargoIntegration.lib.makeOutputs {
      root = ./.;
      overrides.crates = common: _: {
        rust-nix-templater = _: let
          env = {
            TEMPLATER_FMT_BIN = "${common.pkgs.alejandra}/bin/alejandra";
            TEMPLATER_CARGO_BIN = "${common.pkgsWithRust.rustc}/bin/cargo";
          };
        in
          {propagatedEnv = env;} // env;
      };
    };
}
