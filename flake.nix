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
      defaultOutputs = {
        app = "rust-nix-templater";
        package = "rust-nix-templater";
      };
      overrides.crates = common: _: {
        rust-nix-templater = _: let
          env = {
            TEMPLATER_FMT_BIN = "${common.pkgs.alejandra}/bin/alejandra";
            TEMPLATER_CARGO_BIN = "${common.rustToolchain.cargo}/bin/cargo";
            NCI_SRC = toString inputs.nixCargoIntegration;
          };
        in
          {propagatedEnv = env;} // env;
      };
    };
}
