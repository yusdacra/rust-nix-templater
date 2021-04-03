{ common }:
with common; with pkgs;
let
  cachixMetadata = cargoToml.package.metadata.nix.cachix or null;
  cachixName = cachixMetadata.name or null;
  cachixKey = cachixMetadata.key or null;
in
devshell.mkShell {
  packages = [ rustc ] ++ crateDeps.nativeBuildInputs ++ crateDeps.buildInputs;
  commands =
    let
      pkgCmd = pkg: { package = pkg; };
    in
    [
      (pkgCmd git)
      (pkgCmd nixpkgs-fmt)
    ] ++ (lib.optional (!(isNull cachixName)) (pkgCmd cachix));
  env = with lib; [
    (nameValuePair "LD_LIBRARY_PATH" "$LD_LIBRARY_PATH:${lib.makeLibraryPath runtimeLibs}")
  ] ++ (
    optional (!(isNull cachixName) && !(isNull cachixKey))
      (nameValuePair "NIX_CONFIG" ''
        substituters = https://cache.nixos.org https://${cachixName}.cachix.org
        trusted-public-keys = cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY= ${cachixKey}
      '')
  ) ++ (mapAttrsToList nameValuePair env);
}
