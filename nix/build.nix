{ release ? false
, doCheck ? false
, common
,
}:
with common;
let
  meta = with pkgs.stdenv.lib; {
    description = "Rust program to generate Nix files for Rust projects.";
    homepage = "https://github.com/yusdacra/rust-nix-templater";
    license = licenses.mit;
  };

  package = with pkgs; naersk.buildPackage {
    root = ../.;
    nativeBuildInputs = crateDeps.nativeBuildInputs;
    buildInputs = crateDeps.buildInputs;
    overrideMain = (_: {
      inherit meta;
    });
    inherit release doCheck;
  };
in
package
