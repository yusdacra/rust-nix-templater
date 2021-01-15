{ common }:
with common; with pkgs;
mkDevShell {
  packages =
    [ git nixpkgs-fmt cargo rustc {% if cachix_name %} cachix {% endif %} ]
    ++ crateDeps.nativeBuildInputs ++ crateDeps.buildInputs;
  env = {
    {% if cachix_name %}
    NIX_CONFIG = ''
      substituters = https://cache.nixos.org https://{{ cachix_name }}.cachix.org
      trusted-public-keys = cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY= {{ cachix_public_key }}
    '';
    {% endif %}
    LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${lib.makeLibraryPath neededLibs}";
  } // env;
}
