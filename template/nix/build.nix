{ release ? false
, doCheck ? false
, doDoc ? false
, common
,
}:
with common;
let
  meta = with pkgs.lib; {
    description = {% if package_description %} "{{ package_description }}" {% else %} "{{ package_name }} is a Rust project." {% endif %};
    {% if package_long_description %} longDescription = ''{{ package_long_description }}''; {% endif %}
    {% if package_homepage %} homepage = "{{ package_homepage }}"; {% endif %}
    license = licenses.{{ package_license }};
  };

  {% if make_desktop_file %}
  desktopFile = let
    name = "{{ package_name }}";
  in pkgs.makeDesktopItem {
    inherit name;
    exec = "{{ package_executable }}";
    comment = {% if package_xdg_comment %} "{{ package_xdg_comment }}" {% else %} meta.description {% endif %};
    desktopName = {% if package_xdg_desktop_name %} "{{ package_xdg_desktop_name }}" {% else %} name {% endif %};
    {% if package_icon %} icon =  "../{{ package_icon }}"; {% endif %}
    {% if package_xdg_generic_name %} genericName = "{{ package_xdg_generic_name }}"; {% endif %}
    {% if package_xdg_categories %} categories = "{% for category in package_xdg_categories %} {{ category }}; {% endfor %}"; {% endif %}
  };
  {% endif %}

  package = with pkgs; naersk.buildPackage {
    root = ../.;
    nativeBuildInputs = crateDeps.nativeBuildInputs;
    buildInputs = crateDeps.buildInputs;
    # WORKAROUND doctests fail to compile (they compile with nightly cargo but then rustdoc fails)
    cargoTestOptions = def: def ++ [ "--tests" "--bins" "--examples" ];
    override = (prev: (lib.listToAttrs (map (e: { "${e.name}" = e.value; }) env)));
    overrideMain = (prev: {
      inherit meta;
      {% if make_desktop_file %}
      nativeBuildInputs = prev.nativeBuildInputs ++ [ copyDesktopItems wrapGAppsHook ];
      desktopItems = [ desktopFile ];
      {% endif %}
    });
    {% if package_lib %} copyLibs = true; {% endif %}
    inherit release doCheck doDoc;
  };
in
package