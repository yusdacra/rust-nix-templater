{ cargoToml, sources, system }:
let
  pkgz = import sources.nixpkgs { inherit system; overlays = [ sources.rustOverlay.overlay ]; };
  baseRustToolchain =
    if (isNull (cargoToml.package.metadata.nix.toolchain or null))
    then (pkgz.rust-bin.fromRustupToolchainFile "../rust-toolchain")
    else pkgz.rust-bin."${cargoToml.package.metadata.nix.toolchain}".latest.rust;
  rust = baseRustToolchain.override {
    extensions = [ "rust-src" ];
  };

  pkgs = import sources.nixpkgs {
    inherit system;
    overlays = [
      sources.rustOverlay.overlay
      sources.devshell.overlay
      (final: prev: {
        rustc = rust;
      })
      (final: prev: {
        naersk = prev.callPackage sources.naersk { };
      })
    ];
  };
  nixMetadata = cargoToml.package.metadata.nix;
  mapToPkgs = list: map (pkg: pkgs."${pkg}") list;
in
{
  inherit pkgs cargoToml;

  /* You might need this if your application utilizes a GUI. Note that the dependencies
    might change from application to application. The example dependencies provided here
    are for a typical iced application that uses Vulkan underneath.

    For example, it might look like this:

    runtimeLibs = with pkgs; (with xorg; [ libX11 libXcursor libXrandr libXi ])
    ++ [ vulkan-loader wayland wayland-protocols libxkbcommon ];
  */
  runtimeLibs = with pkgs; ([ ] ++ (mapToPkgs (nixMetadata.runtimeLibs or [ ])));

  # Dependencies listed here will be passed to Nix build and development shell
  crateDeps =
    with pkgs;
    {
      buildInputs = [ /* Add runtime dependencies here */ ] ++ (mapToPkgs (nixMetadata.buildInputs or [ ]));
      nativeBuildInputs = [ /* Add compile time dependencies here */ ] ++ (mapToPkgs (nixMetadata.nativeBuildInputs or [ ]));
    };

  /* Put env variables here, like so:

    env = {
      PROTOC = "${pkgs.protobuf}/bin/protoc";
    };

    The variables are not (shell) escaped.
    Variables put here will appear in both dev env and build env.
  */
  env = { } // (nixMetadata.env or { });
}
