{ common }:
with common; with pkgs;
devshell.mkShell {
  packages =
    [ git nixpkgs-fmt rustc cachix ]
    ++ crateDeps.nativeBuildInputs ++ crateDeps.buildInputs;
  env.NIX_CONFIG = ''
    substituters = https://cache.nixos.org https://rust-nix-templater.cachix.org
    trusted-public-keys = cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY= rust-nix-templater.cachix.org-1:Tmy1V0KK+nxzg0XFePL/++t4JRKAw5tvr+FNfHz7mIY=
  '';
}
