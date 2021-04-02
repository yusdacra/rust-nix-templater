#![warn(clippy::pedantic)]
#![allow(clippy::non_ascii_literal)]

mod options;
#[cfg(test)]
mod tests;

use options::{CiType, Options};
use structopt::StructOpt;

use tera::{Context, Tera};

macro_str! {
    GITHUB_CI, ".github/workflows/nix.yml";
    GITLAB_CI, ".gitlab-ci.yml";
    BUILD, "nix/build.nix";
    FLAKE, "flake.nix";
    COMMON, "nix/common.nix";
    DEV, "nix/devShell.nix";
    DEFAULT, "nix/default.nix";
    SHELL, "nix/shell.nix";
    GITIGNORE, ".gitignore";
    ENVRC, "nix/envrc";
}

include_template_files! {
    DEFAULT!(),
    DEV!(),
    SHELL!(),
    ENVRC!(),
    GITIGNORE!(),
    BUILD!(),
    COMMON!(),
    FLAKE!(),
    GITHUB_CI!(),
    GITLAB_CI!(),
}

fn main() {
    let options = Options::from_args();
    run_with_options(options);
}

pub(crate) fn run_with_options(options: Options) {
    let tera = {
        let mut tera = Tera::default();
        tera.add_raw_templates(vec![
            (BUILD!(), get_string!(BUILD!())),
            (FLAKE!(), get_string!(FLAKE!())),
            (COMMON!(), get_string!(COMMON!())),
            (GITHUB_CI!(), get_string!(GITHUB_CI!())),
            (GITLAB_CI!(), get_string!(GITLAB_CI!())),
            (DEV!(), get_string!(DEV!())),
        ])
        .unwrap();
        tera
    };

    let context = build_context_from_opts(&options);

    let out_dir = options.out_dir;

    println!("⚡ Rendering files...");
    let flake_nix = tera.render(FLAKE!(), &context).unwrap();
    let common_nix = tera.render(COMMON!(), &context).unwrap();
    let dev = tera.render(DEV!(), &context).unwrap();

    println!("💾 Writing rendered files...");
    let mut rendered_files = vec![
        (FLAKE!(), flake_nix),
        (COMMON!(), common_nix),
        (DEV!(), dev),
        (SHELL!(), get_string!(SHELL!()).to_owned()),
        (GITIGNORE!(), get_string!(GITIGNORE!()).to_owned()),
        (ENVRC!(), get_string!(ENVRC!()).to_owned()),
    ];

    if !options.disable_build {
        let build_nix = tera.render(BUILD!(), &context).unwrap();
        rendered_files.push((BUILD!(), build_nix));
        rendered_files.push((DEFAULT!(), get_string!(DEFAULT!()).to_owned()));
    }

    for ci in options.ci {
        match ci {
            CiType::Github => {
                let github_ci = tera.render(GITHUB_CI!(), &context).unwrap();
                rendered_files.push((GITHUB_CI!(), github_ci));
            }
            CiType::Gitlab => {
                let gitlab_ci = tera.render(GITLAB_CI!(), &context).unwrap();
                rendered_files.push((GITLAB_CI!(), gitlab_ci));
            }
        }
    }

    write_files(out_dir.as_path(), rendered_files);

    println!("  - Formatting files...");
    let fmt_bin = option_env!("TEMPLATER_FMT_BIN").unwrap_or("nixpkgs-fmt");
    match std::process::Command::new(fmt_bin).arg(&out_dir).output() {
        Ok(_) => println!("  - Format successful: used `{}`", fmt_bin),
        Err(err) => println!(
            "  - Failed to format: error while running `{}`: {}",
            fmt_bin, err
        ),
    }

    if out_dir.join("Cargo.toml").exists() {
        println!("  - Existing Cargo project, not creating a new one");
    } else {
        println!("  - Creating new Cargo project...");
        let cargo_bin = option_env!("TEMPLATER_CARGO_BIN").unwrap_or("cargo");
        match std::process::Command::new(cargo_bin)
            .arg("init")
            .arg(&out_dir)
            .output()
        {
            Ok(_) => {
                println!(
                    "  - Created Cargo project successfully: used `{}`",
                    cargo_bin
                );
                match std::process::Command::new(cargo_bin)
                    .arg("generate-lockfile")
                    .arg("--manifest-path")
                    .arg(out_dir.join("Cargo.toml"))
                    .output()
                {
                    Ok(_) => println!("    - Generated cargo lockfile successfully"),
                    Err(err) => println!("    - Failed to generate cargo lockfile: {}", err),
                }
            }
            Err(err) => println!(
                "  - Failed to create Cargo project: error while running `{}`: {}",
                cargo_bin, err
            ),
        }
    }

    println!("🎉 Finished!");
}

fn write_files(out_dir: &std::path::Path, files: Vec<(&str, String)>) {
    use std::fs;

    // Write files
    for (path, contents) in files {
        let path = out_dir.join(path);
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir).unwrap();
        }
        fs::write(path, contents).unwrap();
    }
}

fn build_context_from_opts(options: &Options) -> Context {
    let mut context = Context::new();

    // Essential variables
    context.insert("package_name", &options.package_name);
    context.insert(
        "package_executable",
        options
            .package_executable
            .as_deref()
            .unwrap_or(&options.package_name),
    );
    context.insert("package_lib", &options.package_lib);
    context.insert("package_license", &options.package_license);
    if let Some(systems) = options.package_systems.as_deref() {
        context.insert("package_systems", systems);
    }
    context.insert("rust_toolchain_file", &options.rust_toolchain_file);
    context.insert(
        "rust_toolchain_channel",
        &options.rust_toolchain_channel.to_string(),
    );
    context.insert("disable_build", &options.disable_build);

    if let Some(desc) = options.package_description.as_deref() {
        context.insert("package_description", desc);
    }
    if let Some(long_desc) = options.package_long_description.as_deref() {
        context.insert("package_long_description", long_desc);
    }
    if let Some(homepage) = options.package_homepage.as_deref() {
        context.insert("package_homepage", homepage);
    }

    let mk_desktop_file = options.make_desktop_file && !options.package_lib;
    context.insert("make_desktop_file", &mk_desktop_file);
    if mk_desktop_file {
        if let Some(icon) = options.package_icon.as_deref() {
            context.insert("package_icon", icon);
        }
        if let Some(comment) = options.package_xdg_comment.as_deref() {
            context.insert("package_xdg_comment", comment);
        }
        if let Some(name) = options.package_xdg_desktop_name.as_deref() {
            context.insert("package_xdg_desktop_name", name);
        }
        if let Some(name) = options.package_xdg_generic_name.as_deref() {
            context.insert("package_xdg_generic_name", name);
        }
        if let Some(categories) = options.package_xdg_categories.as_deref() {
            context.insert("package_xdg_categories", categories);
        }
    }

    if let Some(cachix_name) = options.cachix_name.as_deref() {
        context.insert("cachix_name", cachix_name);
    }
    if let Some(cachix_public_key) = options.cachix_public_key.as_deref() {
        context.insert("cachix_public_key", cachix_public_key);
    }

    context
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
