{ release ? true
, common
,
}:
with common;
let
  {% if make_desktop_file %}
  desktopFile = pkgs.makeDesktopItem rec {
    name = "{{ package_name }}";
    icon = {% if package_icon %} "../{{ package_icon }}" {% else %} "../${name}.ico" {% endif %};
    exec = {% if package_executable %} "{{ package_executable }}" {% else %} name {% endif %};
    comment = {% if package_xdg_comment %} "{{ package_xdg_comment }}" {% elif package_description %} "{{ package_description }}" {% else %} "" {% endif %};
    desktopName = {% if package_xdg_desktop_name %} "{{ package_xdg_desktop_name }}" {% else %} name {% endif %};
    {% if package_xdg_generic_name %} genericName = "{{ package_xdg_generic_name }}"; {% endif %}
    {% if package_xdg_categories %} categories = "{% for category in package_xdg_categories %} {{ category }}; {% endfor %}"; {% endif %}
  };
  {% endif %}

  meta = with pkgs.stdenv.lib; {
    description = {% if package_description %} "{{ package_description }}" {% else %} "Description for {{ package_name }}" {% endif %};
    longDescription = {% if package_long_description %} ''{{ package_long_description }}'' {% else %} ''Long description for {{ package_name }}.'' {% endif %};
    upstream = {% if package_upstream %} "{{ package_upstream }}" {% else %} "https://github.com/<owner>/{{ package_name }}" {% endif %};
    license = licenses.{{ package_license }};
  };

  package = with pkgs; naersk.buildPackage {
    root = ../.;
    nativeBuildInputs = crateDeps.nativeBuildInputs;
    buildInputs = crateDeps.buildInputs;
    overrideMain = (prev: rec {
      inherit meta;
      {% if make_desktop_file %}
      nativeBuildInputs = prev.nativeBuildInputs ++ [ copyDesktopItems wrapGAppsHook ];
      desktopItems = [ desktopFile ];
      {% endif %}
    });
    inherit release;
  };
in
package
