{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    nixCargoIntegration = {
      url = "github:yusdacra/nix-cargo-integration";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs: inputs.nixCargoIntegration.lib.makeOutputs {
    root = ./.;
    overrides = {
      common = prev: {
        env = prev.env // {
          TEMPLATER_FMT_BIN = "${prev.pkgs.nixpkgs-fmt}/bin/nixpkgs-fmt";
          TEMPLATER_CARGO_BIN = "${prev.pkgs.rustc}/bin/cargo";
        };
      };
    };
  };
}
