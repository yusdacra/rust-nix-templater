{ sources, system }:
let
  pkgs = import sources.nixpkgs { inherit system; };
  mozPkgs = import "${sources.nixpkgsMoz}/package-set.nix" { inherit pkgs; };

  rustChannel =
    let
      channel = mozPkgs.rustChannelOf {
        {% if rust_toolchain_file %} rustToolchain = "../rust-toolchain"; {% else %} channel = "{{ rust_toolchain_channel }}"; {% endif %}
        # Replace this with the expected hash that Nix will output when trying to build the package.
        sha256 = pkgs.lib.fakeHash;
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
        inherit (rustChannel);
      })
      (final: prev: {
        naersk = prev.callPackage sources.naersk { };
      })
    ];
  };

  /* You might need this if your application utilizes a GUI. Note that the dependencies
    might change from application to application. The example dependencies provided here
    are for a typical iced application that uses Vulkan underneath.

    For example, it might look like this:

    neededLibs = with pkgs; (with xorg; [ libX11 libXcursor libXrandr libXi ])
    ++ [ vulkan-loader wayland wayland-protocols libxkbcommon ]; */
  neededLibs = [ ];

  # Dependencies listed here will be passed to Nix build and development shell
  crateDeps =
    with pkgs;
    {
      buildInputs = [ /* Add runtime dependencies here */ ];
      nativeBuildInputs = [ /* Add compile time dependencies here */ ];
    };
}
