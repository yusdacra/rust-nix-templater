{ common }:
with common; with pkgs;
mkShell {
  nativeBuildInputs =
    [ git nixpkgs-fmt cargo rustc {% if cachix_name %} cachix {% endif %} ]
    ++ crateDeps.nativeBuildInputs;
  buildInputs = crateDeps.buildInputs;
  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${lib.makeLibraryPath neededLibs}"
    {% if cachix_name %}
    export NIX_CONFIG="
      substituters = https://cache.nixos.org https://{{ cachix_name }}.cachix.org
      trusted-public-keys = cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY= {{ cachix_public_key }}
    " {% endif %}
  '';
}
