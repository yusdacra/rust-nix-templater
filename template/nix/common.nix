{ sources, system }:
let
  pkgz = import sources.nixpkgs { inherit system; };
  mozPkgs = import "${sources.nixpkgsMoz}/package-set.nix" { pkgs = pkgz; };

  rustChannel =
    let
      channel = mozPkgs.rustChannelOf {
        {% if rust_toolchain_file %} rustToolchain = "../rust-toolchain"; {% else %} channel = "{{ rust_toolchain_channel }}"; {% endif %}
        # Replace this with the expected hash that Nix will output when trying to build the package.
        sha256 = pkgz.lib.fakeHash;
      };
    in
    channel // {
      rust = channel.rust.override { extensions = [ "rust-src" "rustfmt-preview" "clippy-preview" ]; };
    };

  pkgs = import sources.nixpkgs {
    inherit system;
    overlays = [
      sources.devshell.overlay
      (final: prev: {
        rustc = rustChannel.rust;
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
