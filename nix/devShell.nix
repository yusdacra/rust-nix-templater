common:
with common; with pkgs;
mkShell {
  name = "rust-nix-templater-devShell";
  nativeBuildInputs =
    [ git nixpkgs-fmt cargo clippy rustc rustfmt ]
    ++ crateDeps.nativeBuildInputs;
  buildInputs = crateDeps.buildInputs;
}
