{
  description = "Flake for {{ package_name }}";

  inputs = {
    devshell.url = "github:numtide/devshell";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    naersk = {
      url = "github:nmattia/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
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
          sources = { inherit devshell naersk nixpkgs nixpkgsMoz; };
          inherit system;
        };

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
        {% if not package_lib %} apps = builtins.mapAttrs (n: v: mkApp { name = n; drv = v; exePath = "/bin/{{ package_executable }}"; }) packages; {% endif %}
      in
      {
        inherit packages {% if not package_lib %} apps {% endif %};

        # Release build is the default package
        defaultPackage = packages."{{ package_name }}";

        {% if not package_lib %}
        # Release build is the default app
        defaultApp = apps."{{ package_name }}";
        {% endif %}

        devShell = import ./nix/devShell.nix { inherit common; };
      }
    );
}
