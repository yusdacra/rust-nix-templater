#![warn(clippy::pedantic)]
#![allow(clippy::non_ascii_literal)]

mod options;

use options::{CiType, Options};
use structopt::StructOpt;

use std::process::Output;

use tera::{Context, Tera};

macro_str! {
    GITHUB_CI, ".github/workflows/nix.yml";
    BUILD, "nix/build.nix";
    FLAKE, "flake.nix";
    COMMON, "nix/common.nix";
    DEV, "nix/devShell.nix";
}

include_template_files! {
    "nix/default.nix",
    DEV!(),
    "nix/shell.nix",
    "nix/envrc",
    ".gitignore",
    BUILD!(),
    COMMON!(),
    FLAKE!(),
    GITHUB_CI!(),
    ;
    GITHUB_CI!(),
}

fn main() {
    let tera = {
        let mut tera = Tera::default();
        tera.add_raw_templates(vec![
            (BUILD!(), get_string!(BUILD!())),
            (FLAKE!(), get_string!(FLAKE!())),
            (COMMON!(), get_string!(COMMON!())),
            (GITHUB_CI!(), get_string!(GITHUB_CI!())),
            (DEV!(), get_string!(DEV!())),
        ])
        .unwrap();
        tera
    };

    let options = Options::from_args();
    let context = build_context_from_opts(&options);

    let out_dir = options.out_dir;

    println!("⚡ Rendering files...");
    let build_nix = tera.render(BUILD!(), &context).unwrap();
    let flake_nix = tera.render(FLAKE!(), &context).unwrap();
    let common_nix = tera.render(COMMON!(), &context).unwrap();
    let github_ci = tera.render(GITHUB_CI!(), &context).unwrap();
    let dev = tera.render(DEV!(), &context).unwrap();

    println!("💾 Writing rendered files...");
    let rendered_files = vec![
        (BUILD!(), build_nix),
        (FLAKE!(), flake_nix),
        (COMMON!(), common_nix),
        (GITHUB_CI!(), github_ci),
        (DEV!(), dev),
    ];
    write_files(out_dir.as_path(), rendered_files, options.ci);

    println!("  - Formatting files...");
    if fmt(out_dir.as_path()).is_ok() {
        println!("  - Format successful!");
    } else {
        println!("  - Failed to format: do you have `nixpkgs-fmt` installed and in your $PATH?");
    }

    println!("🎉 Finished!");
}

fn fmt(out_dir: &std::path::Path) -> std::io::Result<Output> {
    std::process::Command::new("nixpkgs-fmt")
        .arg(out_dir)
        .output()
}

fn write_files(
    out_dir: &std::path::Path,
    mut rendered_files: Vec<(&str, String)>,
    ci_types: Vec<CiType>,
) {
    use std::fs;

    // Create out dir and other dirs we need
    fs::create_dir_all(out_dir.join("nix")).unwrap();

    // Write files we dont need to render
    for (path, contents) in TEMPLATE {
        if OPTIONALS.contains(path) {
            continue;
        }

        let write_to = out_dir.join(path);

        fs::write(write_to, contents).unwrap();
    }

    for ci in ci_types {
        match ci {
            CiType::Github => {
                let pos = rendered_files
                    .iter()
                    .position(|(name, _)| name == &GITHUB_CI!())
                    .unwrap();
                let github_ci = rendered_files.remove(pos).1;
                let path = out_dir.join(GITHUB_CI!());

                fs::create_dir_all(path.parent().unwrap()).unwrap();
                fs::write(path, github_ci).unwrap();
            }
        }
    }

    // Write rendered files
    for (path, contents) in rendered_files {
        if OPTIONALS.contains(&path) {
            continue;
        }

        fs::write(out_dir.join(path), contents).unwrap();
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
    context.insert("package_license", &options.package_license);
    if let Some(systems) = options.package_systems.as_deref() {
        context.insert("package_systems", systems);
    }
    context.insert("rust_toolchain_file", &options.rust_toolchain_file);
    context.insert(
        "rust_toolchain_channel",
        &options.rust_toolchain_channel.to_string(),
    );

    if let Some(desc) = options.package_description.as_deref() {
        context.insert("package_description", desc);
    }
    if let Some(long_desc) = options.package_long_description.as_deref() {
        context.insert("package_long_description", long_desc);
    }
    if let Some(homepage) = options.package_homepage.as_deref() {
        context.insert("package_homepage", homepage);
    }

    context.insert("make_desktop_file", &options.make_desktop_file);
    if options.make_desktop_file {
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
        if let Some(cachix_public_key) = options.cachix_public_key.as_deref() {
            context.insert("cachix_public_key", cachix_public_key);
        }
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
        ;
        $($fopt:expr,)*
    } => {
        const TEMPLATE: &[(&str, &str)] = &[
            $(
                ($f, include_str!(concat!("../template/", $f))),
            )*
        ];

        const OPTIONALS: &[&str] = &[
            $(
                $fopt,
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
