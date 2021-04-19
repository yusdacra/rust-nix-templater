#![warn(clippy::pedantic)]
#![allow(clippy::non_ascii_literal, clippy::too_many_lines)]

pub mod options;
#[cfg(test)]
mod tests;

pub use options::Options;
pub use structopt::StructOpt;

use anyhow::bail;
use options::CiType;
use std::fmt::Display;

macro_str! {
    GITHUB_CI, ".github/workflows/nix.yml";
    GITLAB_CI, ".gitlab-ci.yml";
    FLAKE, "flake.nix";
    DEFAULT, "default.nix";
    SHELL, "shell.nix";
    GITIGNORE, "gitignore";
    ENVRC, ".envrc";
}

include_template_files! {
    DEFAULT!(),
    SHELL!(),
    ENVRC!(),
    GITIGNORE!(),
    FLAKE!(),
    GITHUB_CI!(),
    GITLAB_CI!(),
}

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

pub fn run_with_options(options: Options, print_msg: bool) -> anyhow::Result<()> {
    let out_dir = options.out_dir;
    let cargo_toml_path = out_dir.join("Cargo.toml");
    let has_project = cargo_toml_path.exists();

    if has_project {
        if print_msg {
            println!("- Existing Cargo project detected.")
        }
    } else {
        if print_msg {
            println!("- No Cargo project detected.");
        }
        if options.package_name.is_none() {
            if print_msg {
                println!(
                    "  - You must pass a project name while creating a Cargo project, aborting."
                );
            }
            bail!("must set a project name while creating a cargo project");
        }
    }

    if print_msg {
        println!("💾 Writing files...");
    }
    let mut rendered_files =
        std::array::IntoIter::new([FLAKE!(), SHELL!(), GITIGNORE!(), ENVRC!(), DEFAULT!()])
            .map(|name| (name, get_string!(name).to_owned()))
            .collect::<Vec<_>>();

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

    for ci in options.ci {
        match ci {
            CiType::Github => cachix_render(GITHUB_CACHIX, GITHUB_CI!()),
            CiType::Gitlab => cachix_render(GITLAB_CACHIX, GITLAB_CI!()),
        }
    }

    write_files(out_dir.as_path(), rendered_files)?;

    if print_msg {
        println!("  - Formatting files...");
    }
    let fmt_bin = option_env!("TEMPLATER_FMT_BIN").unwrap_or("nixpkgs-fmt");
    match std::process::Command::new(fmt_bin).arg(&out_dir).output() {
        Ok(_) => {
            if print_msg {
                println!("  - Format successful: used `{}`", fmt_bin)
            }
        }
        Err(err) => {
            if print_msg {
                println!(
                    "  - Failed to format: error while running `{}`: {}",
                    fmt_bin, err
                )
            }
        }
    }

    if !has_project {
        if print_msg {
            println!("  - Creating new Cargo project...");
        }
        let cargo_bin = option_env!("TEMPLATER_CARGO_BIN").unwrap_or("cargo");
        match std::process::Command::new(cargo_bin)
            .arg("init")
            .arg(&out_dir)
            .output()
        {
            Ok(_) => {
                if print_msg {
                    println!(
                        "  - Created Cargo project successfully: used `{}`",
                        cargo_bin
                    );
                }
                match std::process::Command::new(cargo_bin)
                    .arg("generate-lockfile")
                    .arg("--manifest-path")
                    .arg(&cargo_toml_path)
                    .output()
                {
                    Ok(_) => {
                        if print_msg {
                            println!("    - Generated Cargo lockfile successfully")
                        }
                    }
                    Err(err) => {
                        if print_msg {
                            println!("    - Failed to generate Cargo lockfile: {}", err)
                        }
                    }
                }
            }
            Err(err) => {
                if print_msg {
                    println!(
                        "  - Failed to create Cargo project: error while running `{}`: {}",
                        cargo_bin, err
                    );
                }
                bail!("failed to create cargo project: {}", err);
            }
        }
    }

    let mut cargo_toml = std::fs::read_to_string(&cargo_toml_path)?
        .lines()
        .map(str::to_string)
        .collect::<Vec<_>>();

    let index = cargo_toml
        .iter()
        .position(|line| line.contains("name = "))
        .map(|i| i + 1);

    if let Some(mut index) = index {
        if !cargo_toml.iter().any(|line| line.contains("license = ")) {
            if let Some(license) = &options.package_license {
                cargo_toml.insert(index, format!("license = \"{}\"", license));
                index += 1;
            }
        }
        if !cargo_toml
            .iter()
            .any(|line| line.contains("description = "))
        {
            if let Some(description) = &options.package_description {
                cargo_toml.insert(index, format!("description = \"{}\"", description));
            }
        }
    }

    let parent = index.is_some().then(|| "package").unwrap_or("workspace");

    cargo_toml.push(format!("\n[{}.metadata.nix]", parent));
    cargo_toml.push("# Toggle app flake output".to_owned());
    cargo_toml.kv("app", !options.disable_app);
    cargo_toml.push("# Toggle flake outputs that build (checks, package and app)".to_owned());
    cargo_toml.kv("build", !options.disable_build);
    cargo_toml.push("# Whether to copy built library to package output".to_owned());
    cargo_toml.kv("library", options.package_lib);
    if let Some(long_description) = &options.package_long_description {
        cargo_toml.kv("longDescription", quote(long_description));
    }
    if let Some(executable) = &options.package_executable {
        cargo_toml.push("# Executable name to be used for app output".to_owned());
        cargo_toml.kv("executable", quote(executable));
    }
    if !options.rust_toolchain_file {
        cargo_toml.push("# Toolchain to be used".to_owned());
        cargo_toml.kv("toolchain", quote(options.rust_toolchain_channel));
    }
    if let Some(cachix_name) = &options.cachix_name {
        cargo_toml.push(format!("\n[{}.metadata.nix.cachix]", parent));
        cargo_toml.push("# Name of the cachix binary cache".to_owned());
        cargo_toml.kv("name", quote(cachix_name));
        if let Some(cachix_key) = &options.cachix_public_key {
            cargo_toml.push("# Public key of this cache".to_owned());
            cargo_toml.kv("key", quote(cachix_key));
        }
    }
    if options.make_desktop_file {
        cargo_toml.push(format!("\n[{}.metadata.nix.desktopFile]", parent));
        if let Some(desktop_name) = &options.package_xdg_desktop_name {
            cargo_toml.kv("name", quote(desktop_name));
        }
        if let Some(generic_name) = &options.package_xdg_generic_name {
            cargo_toml.kv("genericName", quote(generic_name));
        }
        if let Some(categories) = &options.package_xdg_categories {
            cargo_toml.kv(
                "categories",
                quote(
                    categories
                        .iter()
                        .map(|c| format!("{}; ", c))
                        .collect::<String>(),
                ),
            );
        }
        if let Some(comment) = &options.package_xdg_comment {
            cargo_toml.kv("comment", quote(comment));
        }
        if let Some(icon) = &options.package_icon {
            cargo_toml.kv("icon", quote(icon));
        }
    }
    let new_cargo_toml = cargo_toml
        .into_iter()
        .map(|mut c| {
            c.reserve(1);
            c.push('\n');
            c
        })
        .collect::<String>();
    std::fs::write(&cargo_toml_path, new_cargo_toml.into_bytes())?;

    if print_msg {
        println!("🎉 Finished!");
    }

    Ok(())
}

trait TomlExt {
    fn kv(&mut self, key: impl Display, value: impl Display);
}

impl TomlExt for Vec<String> {
    fn kv(&mut self, key: impl Display, value: impl Display) {
        self.push(format!("{} = {}", key, value));
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
