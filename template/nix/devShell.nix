{ common }:
with common; with pkgs;
devshell.mkShell {
  packages = [ rustc ] ++ crateDeps.nativeBuildInputs ++ crateDeps.buildInputs;
  commands = let
    pkgCmd = pkg: { package = pkg; };
  in [
    (pkgCmd git)
    (pkgCmd nixpkgs-fmt)
    {% if cachix_name %} (pkgCmd cachix) {% endif %}
  ];
  env = with lib; [
    {% if cachix_name and cachix_public_key %}
    (
      nameValuePair "NIX_CONFIG" ''
        substituters = https://cache.nixos.org https://{{ cachix_name }}.cachix.org
        trusted-public-keys = cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY= {{ cachix_public_key }}
      ''
    )
    {% endif %}
    (nameValuePair "LD_LIBRARY_PATH" "$LD_LIBRARY_PATH:${lib.makeLibraryPath neededLibs}")
  ] ++ env;
}
