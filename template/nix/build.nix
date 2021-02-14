{ release ? false
, doCheck ? false
, doDoc ? false
, common
,
}:
with common;
let
  meta = with pkgs.stdenv.lib; {
    description = {% if package_description %} "{{ package_description }}" {% else %} "Description for {{ package_name }}" {% endif %};
    longDescription = {% if package_long_description %} ''{{ package_long_description }}'' {% else %} ''Long description for {{ package_name }}.'' {% endif %};
    homepage = {% if package_homepage %} "{{ package_homepage }}" {% else %} "https://github.com/<owner>/{{ package_name }}" {% endif %};
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
    # WORKAROUND because doctests fail to compile (they compile with nightly cargo but then rustdoc fails)
    cargoTestOptions = def: def ++ [ "--lib" "--tests" "--bins" "--examples" ];
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