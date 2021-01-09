{ sources, system }:
let
  pkgs = import sources.nixpkgs { inherit system; };
  mozPkgs = import "${sources.nixpkgsMoz}/package-set.nix" { inherit pkgs; };

  rustChannel =
    let
      channel = mozPkgs.rustChannelOf {
        channel = "stable";
        sha256 = "sha256-KCh2UBGtdlBJ/4UOqZlxUtcyefv7MH1neoVNV4z0nWs=";
      };
    in
    channel // {
      rust = channel.rust.override { extensions = [ "rust-src" "rustfmt-preview" "clippy-preview" ]; };
    };
in
rec {
  pkgs = import sources.nixpkgs {
    inherit system;
    overlays = [
      (final: prev: {
        rustc = rustChannel.rust;
        clippy = rustChannel.clippy-preview;
        rustfmt = rustChannel.rustfmt-preview;
      })
      (final: prev: {
        naersk = prev.callPackage sources.naersk { };
      })
    ];
  };

  crateDeps =
    with pkgs;
    {
      buildInputs = [ ];
      nativeBuildInputs = [ ];
    };
}
