{
  description = "Flake for {{ package_name }}";

  inputs = rec {
    naersk = {
      url = "github:nmattia/naersk";
      inputs.nixpkgs = nixpkgs;
    };
    flakeUtils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    nixpkgsMoz = {
      url = "github:mozilla/nixpkgs-mozilla";
      flake = false;
    };
  };

  outputs = inputs: with inputs;
    with flakeUtils.lib;
    eachSystem {% if package_systems %} [ {% for system in package_systems %} "{{ system }}" {% endfor %} ] {% else %} defaultSystems {% endif %} (system:
      let
        common = import ./nix/common.nix {
          sources = { inherit naersk nixpkgs nixpkgsMoz; };
          inherit system;
        };
      in
      rec {
        packages = {
          "{{ package_name }}" = import ./nix/build.nix { inherit common; };
          "{{ package_name }}-debug" = import ./nix/build.nix { inherit common; release = false; };
        };
        defaultPackage = packages."{{ package_name }}";

        apps = builtins.mapAttrs (n: v: mkApp { name = n; drv = v; exePath = "/bin/{{ package_executable }}"; }) packages;
        defaultApp = apps."{{ package_name }}";

        devShell = (import ./nix/devShell.nix) common;
      }
    );
}
