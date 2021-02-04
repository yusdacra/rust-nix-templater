{ sources, system }:
let
  pkgz = import sources.nixpkgs { inherit system; overlays = [ sources.rustOverlay.overlay ]; };
  rust = {% if rust_toolchain_file %} (pkgz.rust-bin.fromRustupToolchainFile "../rust-toolchain") {% else %} pkgz.rust-bin."{{ rust_toolchain_channel }}".latest.rust {% endif %}.override {
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
in
{
  inherit pkgs;

  /* You might need this if your application utilizes a GUI. Note that the dependencies
    might change from application to application. The example dependencies provided here
    are for a typical iced application that uses Vulkan underneath.

    For example, it might look like this:

    neededLibs = with pkgs; (with xorg; [ libX11 libXcursor libXrandr libXi ])
    ++ [ vulkan-loader wayland wayland-protocols libxkbcommon ];
  */
  neededLibs = [ ];

  # Dependencies listed here will be passed to Nix build and development shell
  crateDeps =
    with pkgs;
    {
      buildInputs = [ /* Add runtime dependencies here */ ];
      nativeBuildInputs = [ /* Add compile time dependencies here */ ];
    };
  
  /* Put env variables here, like so:

    env = {
      PROTOC = "${pkgs.protobuf}/bin/protoc";
    };

    The variables are not (shell) escaped.
    Variables put here will appear in both dev env and build env.
  */
  env = { };
}
