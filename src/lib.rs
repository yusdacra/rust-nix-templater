#![warn(clippy::pedantic)]
#![allow(clippy::non_ascii_literal, clippy::too_many_lines)]

pub mod options;
#[cfg(test)]
mod tests;

pub use options::Options;
pub use structopt::StructOpt;

use anyhow::bail;
use std::{fmt::Display, ops::Not, process::Command};

macro_str! {
    GITHUB_CI, ".github/workflows/nix.yml";
    GITLAB_CI, ".gitlab-ci.yml";
    FLAKE, concat!(env!("NCI_SRC"), "/docs/example_flake.nix");
    DEFAULT, "default.nix";
    SHELL, "shell.nix";
    COMPAT, "compat.nix";
    GITIGNORE, "gitignore";
    ENVRC, ".envrc";
}

include_template_files! {
    DEFAULT!(),
    SHELL!(),
    COMPAT!(),
    ENVRC!(),
    GITIGNORE!(),
    GITHUB_CI!(),
    GITLAB_CI!(),
}

#[cfg(test)]
const BASE_FILES: [&str; 6] = [
    "flake.nix",
    SHELL!(),
    GITIGNORE!(),
    ENVRC!(),
    DEFAULT!(),
    COMPAT!(),
];

const FLAKE_CONTENTS: &str = include_str!(FLAKE!());

const GITHUB_CACHIX: &str = r"    - name: Cachix cache
      uses: cachix/cachix-action@v10
      with:
        name: cachix_name
        authToken: '${{ secrets.CACHIX_AUTH_TOKEN }}'
";

const GITLAB_CACHIX: &str = r"
variables:
  CACHIX_NAME: cachix_name
before_script:
  - . /bin/pre-build.sh
after_script:
  - . /bin/post-build.sh
";

/// Run with options.
///
/// # Errors
/// Will error if any IO error occurs, or if the passed options are invalid.
pub fn run_with_options(options: Options, should_print_msg: bool) -> anyhow::Result<()> {
    let out_dir = options.out_dir;
    let cargo_toml_path = out_dir.join("Cargo.toml");
    let has_project = cargo_toml_path.exists();
    let print_msg = |msg: &str| {
        if should_print_msg {
            println!("{}", msg)
        }
    };

    if has_project {
        print_msg("- Existing Cargo project detected.");
    } else {
        print_msg("- No Cargo project detected.");
        if options.package_name.is_none() {
            print_msg("  - You must pass a project name while creating a Cargo project, aborting.");
            bail!("must set a project name while creating a cargo project");
        }
    }

    print_msg("💾 Writing files...");
    let mut rendered_files = [SHELL!(), GITIGNORE!(), ENVRC!(), DEFAULT!(), COMPAT!()]
        .into_iter()
        .map(|name| (name, get_string!(name).to_owned()))
        .collect::<Vec<_>>();

    rendered_files.push(("flake.nix", FLAKE_CONTENTS.to_string()));

    let cachix_name = &options.cachix_name;
    let mut cachix_render = |replacement: &str, filename| {
        let github_file = get_string!(filename);
        let rendered = github_file.replace(
            replacement,
            &cachix_name.as_ref().map_or_else(
                || "".to_owned(),
                |cachix_name| replacement.replace("cachix_name", cachix_name),
            ),
        );
        rendered_files.push((filename, rendered));
    };

    if options.github_ci {
        cachix_render(GITHUB_CACHIX, GITHUB_CI!());
    }
    if options.gitlab_ci {
        cachix_render(GITLAB_CACHIX, GITLAB_CI!());
    }

    write_files(out_dir.as_path(), rendered_files)?;

    print_msg("  - Formatting files...");
    let fmt_bin = option_env!("TEMPLATER_FMT_BIN").unwrap_or("nixpkgs-fmt");
    match Command::new(fmt_bin).arg(&out_dir).output() {
        Ok(_) => {
            print_msg(&format!("  - Format successful: used `{}`", fmt_bin));
        }
        Err(err) => {
            print_msg(&format!(
                "  - Failed to format: error while running `{}`: {}",
                fmt_bin, err
            ));
        }
    }

    if !has_project {
        print_msg("  - Creating new Cargo project...");
        let cargo_bin = option_env!("TEMPLATER_CARGO_BIN").unwrap_or("cargo");
        match Command::new(cargo_bin).arg("init").arg(&out_dir).output() {
            Ok(_) => {
                print_msg(&format!(
                    "  - Created Cargo project successfully: used `{}`",
                    cargo_bin
                ));
                match Command::new(cargo_bin)
                    .arg("generate-lockfile")
                    .arg("--manifest-path")
                    .arg(&cargo_toml_path)
                    .output()
                {
                    Ok(_) => {
                        print_msg("    - Generated Cargo lockfile successfully");
                    }
                    Err(err) => {
                        print_msg(&format!("    - Failed to generate Cargo lockfile: {}", err));
                    }
                }
            }
            Err(err) => {
                print_msg(&format!(
                    "  - Failed to create Cargo project: error while running `{}`: {}",
                    cargo_bin, err
                ));
                bail!("failed to create cargo project: {}", err);
            }
        }
    }

    let mut cargo_toml = std::fs::read_to_string(&cargo_toml_path)?;

    let index = cargo_toml.find("name = ");

    if let Some(mut index) = index {
        if cargo_toml.contains("license = ").not() {
            if let Some(license) = &options.package_license {
                const LICENSE_KEY: &str = "license = \"";
                const LICENSE_KEY_END: &str = "\"\n";

                cargo_toml.insert_str(index, LICENSE_KEY);
                index += LICENSE_KEY.len();
                cargo_toml.insert_str(index, license);
                index += license.len();
                cargo_toml.insert_str(index, LICENSE_KEY_END);
                index += LICENSE_KEY_END.len();
            }
        }
        if cargo_toml.contains("description = ").not() {
            if let Some(description) = &options.package_description {
                const DESC_KEY: &str = "description = \"";
                const DESC_KEY_END: &str = "\"\n";

                cargo_toml.insert_str(index, DESC_KEY);
                index += DESC_KEY.len();
                cargo_toml.insert_str(index, description);
                index += description.len();
                cargo_toml.insert_str(index, DESC_KEY_END);
            }
        }
    }

    let parent = index.map_or("workspace", |_| "package");

    cargo_toml.push('\n');
    cargo_toml.attrset(format!("{}.metadata.nix", parent));
    cargo_toml.comment("Toggle app flake output");
    cargo_toml.kv(
        "app",
        (options.disable_app.not() && options.disable_build.not()).to_str(),
    );
    cargo_toml.comment("Toggle flake outputs that build (checks, package and app)");
    cargo_toml.kv("build", options.disable_build.not().to_str());
    if let Some(long_description) = &options.package_long_description {
        cargo_toml.kv("longDescription", quote(long_description));
    }
    if options.rust_toolchain_channel != options::RustToolchainChannel::default() {
        cargo_toml.comment("Toolchain to be used");
        cargo_toml.kv("toolchain", quote(options.rust_toolchain_channel));
    }
    if let Some(cachix_name) = &options.cachix_name {
        cargo_toml.attrset(format!("{}.metadata.nix.cachix", parent));
        cargo_toml.comment("Name of the cachix binary cache");
        cargo_toml.kv("name", quote(cachix_name));
        if let Some(cachix_key) = &options.cachix_public_key {
            cargo_toml.comment("Public key of this cache");
            cargo_toml.kv("key", quote(cachix_key));
        }
    }
    if options.package_xdg_desktop_name.is_some()
        || options.package_xdg_generic_name.is_some()
        || options.package_xdg_categories.is_some()
        || options.package_xdg_comment.is_some()
        || options.package_icon.is_some()
    {
        cargo_toml.attrset(format!("{}.metadata.nix.desktopFile", parent));
        if let Some(desktop_name) = &options.package_xdg_desktop_name {
            cargo_toml.kv("name", quote(desktop_name));
        }
        if let Some(generic_name) = &options.package_xdg_generic_name {
            cargo_toml.kv("genericName", quote(generic_name));
        }
        if let Some(categories) = &options.package_xdg_categories {
            cargo_toml.kv("categories", quote(categories));
        }
        if let Some(comment) = &options.package_xdg_comment {
            cargo_toml.kv("comment", quote(comment));
        }
        if let Some(icon) = &options.package_icon {
            cargo_toml.kv("icon", quote(icon));
        }
    }
    std::fs::write(&cargo_toml_path, cargo_toml.into_bytes())?;

    print_msg("🎉 Finished!");

    Ok(())
}

trait TomlExt {
    fn kv(&mut self, key: impl AsRef<str>, value: impl AsRef<str>);
    fn comment(&mut self, comment: impl AsRef<str>);
    fn attrset(&mut self, name: impl AsRef<str>);
}

impl TomlExt for String {
    fn kv(&mut self, key: impl AsRef<str>, value: impl AsRef<str>) {
        let key = key.as_ref();
        let value = value.as_ref();

        self.reserve(key.len() + value.len() + 4);
        self.push_str(key);
        self.push_str(" = ");
        self.push_str(value);
        self.push('\n');
    }

    fn comment(&mut self, comment: impl AsRef<str>) {
        let comment = comment.as_ref();

        self.reserve(comment.len() + 3);
        self.push_str("# ");
        self.push_str(comment);
        self.push('\n');
    }

    fn attrset(&mut self, name: impl AsRef<str>) {
        let name = name.as_ref();

        self.reserve(name.len() + 3);
        self.push('[');
        self.push_str(name);
        self.push(']');
        self.push('\n');
    }
}

trait BoolExt {
    fn to_str(&self) -> &'static str;
}

impl BoolExt for bool {
    fn to_str(&self) -> &'static str {
        if *self {
            "true"
        } else {
            "false"
        }
    }
}

fn quote(value: impl Display) -> String {
    format!("\"{}\"", value)
}

fn write_files(out_dir: &std::path::Path, files: Vec<(&str, String)>) -> anyhow::Result<()> {
    use std::fs;

    // Write files
    for (path, contents) in files {
        let path = if path == "gitignore" {
            ".gitignore"
        } else {
            path
        };
        let path = out_dir.join(path);
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        fs::write(path, contents)?;
    }

    Ok(())
}

#[macro_export]
macro_rules! get_string {
    ($filename:expr) => {
        TEMPLATE
            .iter()
            .find(|(name, _)| name == &$filename)
            .unwrap()
            .1
    };
}

#[macro_export]
macro_rules! include_template_files {
    {
        $($f:expr,)*
    } => {
        const TEMPLATE: &[(&str, &str)] = &[
            $(
                ($f, include_str!(concat!("../template/", $f))),
            )*
        ];
    };
}

#[macro_export]
macro_rules! macro_str {
    {
        $( $name:ident, $str:expr; )*
    } => {
        $(
            macro_rules! $name {
                () => {
                    $str
                };
            }
        )*
    };
}
