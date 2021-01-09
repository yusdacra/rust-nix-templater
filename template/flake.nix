{
  description = "Flake for {{ package_name }}";

  inputs = let
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    };
  in {
    inherit nixpkgs;
    naersk = {
      url = "github:nmattia/naersk";
      inputs.nixpkgs = nixpkgs;
    };
    flakeUtils.url = "github:numtide/flake-utils";
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
          # Compiles slower but has tests and faster executable
          "{{ package_name }}" = import ./nix/build.nix {
            inherit common;
            doCheck = true;
            release = true;
          };
          # Compiles faster but no tests and slower executable
          "{{ package_name }}-debug" = import ./nix/build.nix { inherit common; };
          # Compiles faster but has tests and slower executable
          "{{ package_name }}-tests" = import ./nix/build.nix { inherit common; doCheck = true; };
        };
        # Release build is the default package
        defaultPackage = packages."{{ package_name }}";

        apps = builtins.mapAttrs (n: v: mkApp { name = n; drv = v; exePath = "/bin/{{ package_executable }}"; }) packages;
        # Release build is the default app
        defaultApp = apps."{{ package_name }}";

        devShell = import ./nix/devShell.nix { inherit common; };
      }
    );
}
