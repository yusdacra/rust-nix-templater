{
  description = "Flake for rust-nix-templater";

  inputs = rec {
    naersk = {
      url = "github:nmattia/naersk";
      inputs.nixpkgs = nixpkgs;
    };
    flakeUtils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs";
    nixpkgsMoz = {
      url = "github:mozilla/nixpkgs-mozilla";
      flake = false;
    };
  };

  outputs = inputs: with inputs;
    with flakeUtils.lib;
    eachSystem defaultSystems (system:
      let
        common = import ./nix/common.nix {
          sources = { inherit naersk nixpkgs nixpkgsMoz; };
          inherit system;
        };
      in
      rec {
        packages = {
          rust-nix-templater = import ./nix/build.nix { inherit common; };
          rust-nix-templater-debug = import ./nix/build.nix { inherit common; release = false; };
        };
        defaultPackage = packages.rust-nix-templater;

        apps = builtins.mapAttrs (n: v: mkApp { name = n; drv = v; exePath = "/bin/rust-nix-templater"; }) packages;
        defaultApp = apps.rust-nix-templater;

        devShell = (import ./nix/devShell.nix) common;
      }
    );
}
