{
  description = "Flake for {{ package_name }}";

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
    eachSystem [ {% for system in package_systems %} "{{ system }}" {% endfor %} ] (system:
      let
        common = import ./nix/common.nix {
          sources = { inherit naersk nixpkgs nixpkgsMoz; };
          inherit system;
        };
      in
      rec {
        packages = {
          "{{ package_name }}" = import ./nix/build.nix { inherit common; };
        };
        defaultPackage = packages."{{ package_name }}";

        apps = builtins.mapAttrs (n: v: mkApp { name = n; drv = v; }) packages;
        defaultApp = apps."{{ package_name }}";

        devShell = (import ./nix/devShell.nix) common;
      }
    );
}
