{ release ? false
, doCheck ? false
, doDoc ? false
, common
,
}:
with common;
let
  tomlDesktopFile = cargoToml.package.metadata.nix.xdg or null;
  makeDesktopFile = tomlDesktopFile.enable or false;

  meta = with pkgs.lib; ({
    description = cargoToml.package.description or "${cargoToml.package.name} is a Rust project.";
  } // (optionalAttrs (builtins.hasAttr "license" cargoToml.package) { license = licenses."${toLower cargoToml.package.license}"; })
  // (optionalAttrs (builtins.hasAttr "homepage" cargoToml.package) { inherit (cargoToml.package) homepage; })
  // (optionalAttrs (builtins.hasAttr "longDescription" cargoToml.package.metadata.nix) { inherit (cargoToml.package.metadata.nix) longDescription; }));

  desktopFile =
    with pkgs.lib;
    let
      name = cargoToml.package.name;
      makeIcon = icon:
        if (hasPrefix "./" icon)
        then "${../.}/${removePrefix "./" icon}"
        else icon;
    in
    ((pkgs.makeDesktopItem {
      inherit name;
      exec = cargoToml.package.metadata.nix.executable or name;
      comment = tomlDesktopFile.comment or meta.description;
      desktopName = tomlDesktopFile.name or name;
    }) // (optionalAttrs (builtins.hasAttr "icon" tomlDesktopFile) { icon = makeIcon tomlDesktopFile.icon; })
    // (optionalAttrs (builtins.hasAttr "genericName" tomlDesktopFile) { inherit (tomlDesktopFile) genericName; })
    // (optionalAttrs (builtins.hasAttr "categories" tomlDesktopFile) { inherit (tomlDesktopFile) categories; }));

  package = with pkgs;
    let
      library = cargoToml.package.metadata.nix.library or false;
    in
    naersk.buildPackage {
      root = ../.;
      nativeBuildInputs = crateDeps.nativeBuildInputs;
      buildInputs = crateDeps.buildInputs;
      # WORKAROUND doctests fail to compile (they compile with nightly cargo but then rustdoc fails)
      cargoTestOptions = def: def ++ [ "--tests" "--bins" "--examples" ] ++ (lib.optional library "--lib");
      override = (prev: env);
      overrideMain = (prev: {
        inherit meta;
      } // (
        lib.optionalAttrs makeDesktopFile
          { nativeBuildInputs = prev.nativeBuildInputs ++ [ copyDesktopItems wrapGAppsHook ]; desktopItems = [ desktopFile ]; }
      ));
      copyLibs = library;
      inherit release doCheck doDoc;
    };
in
package
