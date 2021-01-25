{ sources, system }:
let
  pkgz = import sources.nixpkgs { inherit system; overlays = [ sources.rustOverlay.overlay ]; };
  rustChannel = pkgz.rust-bin.stable.latest;

  pkgs = import sources.nixpkgs {
    inherit system;
    overlays = [
      sources.rustOverlay.overlay
      sources.devshell.overlay
      (final: prev: {
        rustc = rustChannel.rust.override {
          extensions = [ "rust-src" ];
        };
      })
      (final: prev: {
        naersk = prev.callPackage sources.naersk { };
      })
    ];
  };
in
{
  inherit pkgs;

  crateDeps =
    with pkgs;
    {
      buildInputs = [ ];
      nativeBuildInputs = [ ];
    };
}
