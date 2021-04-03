{
  inputs = {
    devshell.url = "github:numtide/devshell";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    naersk = {
      url = "github:nmattia/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flakeUtils.url = "github:numtide/flake-utils";
    rustOverlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs: with inputs;
    with flakeUtils.lib;
    let
      cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
    in
    eachSystem (cargoToml.package.metadata.nix.systems or defaultSystems) (system:
      let
        common = import ./nix/common.nix {
          sources = { inherit devshell naersk nixpkgs rustOverlay; };
          inherit system cargoToml;
        };

        lib = common.pkgs.lib;
        packages = {
          # Compiles slower but has tests and faster executable
          "${cargoToml.package.name}" = import ./nix/build.nix {
            inherit common;
            doCheck = true;
            release = true;
          };
          # Compiles faster but no tests and slower executable
          "${cargoToml.package.name}-debug" = import ./nix/build.nix { inherit common; };
        };
        checks = {
          # Compiles faster but has tests and slower executable
          "${cargoToml.package.name}-tests" = import ./nix/build.nix { inherit common; doCheck = true; };
        };
        apps = builtins.mapAttrs (n: v: mkApp { name = n; drv = v; exePath = "/bin/${cargoToml.package.metadata.nix.executable or cargoToml.package.name}"; }) packages;
      in
      ({
        devShell = import ./nix/devShell.nix { inherit common; };
      } // (lib.optionalAttrs (cargoToml.package.metadata.nix.build or false) ({
        inherit packages checks;
        # Release build is the default package
        defaultPackage = packages."${cargoToml.package.name}";
      } // (lib.optionalAttrs (cargoToml.package.metadata.nix.app or false) {
        inherit apps;
        # Release build is the default app
        defaultApp = apps."${cargoToml.package.name}";
      }))))
    );
}
