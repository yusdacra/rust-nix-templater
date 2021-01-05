use include_dir::Dir;
use structopt::StructOpt;
use tera::{Context, Tera};

const TEMPLATE: Dir = include_dir::include_dir!("template");
const NOT_RENDERED_FILE_PATHS: &[&str] = &[
    "nix/common.nix",
    "nix/default.nix",
    "nix/devShell.nix",
    "nix/shell.nix",
    "nix/envrc",
    ".gitignore",
];

#[derive(StructOpt, Debug)]
#[structopt(name = "rust-nix-templater")]
struct Options {
    /// Create a desktop file
    #[structopt(long = "mk-desktop-file")]
    make_desktop_file: bool,

    /// The output dir where rendered files will be put in.
    #[structopt(short, long)]
    out_dir: Option<String>,

    /// Name of the package.
    #[structopt(parse(from_str))]
    package_name: String,
    /// A short, single line description of the package.
    #[structopt(long = "pkg-desc")]
    package_description: Option<String>,
    /// A longer description of the package.
    #[structopt(long = "pkg-desc-long")]
    package_long_description: Option<String>,
    /// Systems that the package is supported on. [example: x86_64-linux,x86-linux]
    #[structopt(long = "pkg-sys", default_value = "x86_64-linux")]
    package_systems: Option<Vec<String>>,
    /// Upstream repository of the package. [example: "https://gitlab.com/example/example"]
    #[structopt(long = "pkg-repo")]
    package_upstream: Option<String>,
    /// License of the package. Can be any of the values listed in https://github.com/NixOS/nixpkgs/blob/master/lib/licenses.nix.
    #[structopt(long = "pkg-license", default_value = "gpl3")]
    package_license: String,

    /// Only required if your package's executable name is different from your package's name.
    #[structopt(long = "pkg-exec")]
    package_executable: Option<String>,
}

fn main() {
    // Make sure we can parse our templates
    let tera = {
        let mut tera = Tera::default();
        tera.add_raw_templates(vec![
            ("build.nix", get_string!("nix/build.nix")),
            ("flake.nix", get_string!("flake.nix")),
        ])
        .unwrap();
        tera
    };

    // Get options
    let options = Options::from_args();

    // Construct Context from options
    let context = build_context_from_opts(&options);

    // Get out dir from options
    let out_dir = std::path::PathBuf::from(options.out_dir.unwrap_or("out".to_string()));

    // Render files
    println!("⚡ Rendering files...");
    let build_nix = tera.render("build.nix", &context).unwrap();
    let flake_nix = tera.render("flake.nix", &context).unwrap();

    println!("💾 Writing rendered files...");
    let rendered_files = vec![("nix/build.nix", build_nix), ("flake.nix", flake_nix)];
    write_files(out_dir.as_path(), rendered_files);

    println!("  - Formatting files...");
    try_fmt(out_dir.as_path());

    println!("🎉 Finished!");
}

fn try_fmt(out_dir: &std::path::Path) {
    if std::process::Command::new("nixpkgs-fmt")
        .arg(out_dir)
        .output()
        .is_ok()
    {
        println!("  - Format successful!");
    } else {
        println!("  - Failed to format: do you have `nixpkgs-fmt` installed an in your $PATH?");
    }
}

fn write_files(out_dir: &std::path::Path, rendered_files: Vec<(&str, String)>) {
    use std::fs;

    // Create out dir and other dirs we need
    fs::create_dir_all(out_dir.join("nix")).unwrap();

    // Write files we dont need to render
    for path in NOT_RENDERED_FILE_PATHS {
        let contents = get_string!(&path);
        let write_to = out_dir.join(path);

        fs::write(write_to, contents).unwrap();
    }

    // Write rendered files
    for (path, contents) in rendered_files {
        fs::write(out_dir.join(path), contents).unwrap();
    }
}

fn build_context_from_opts(options: &Options) -> Context {
    let mut context = Context::new();
    context.insert("package_name", &options.package_name);

    if let Some(desc) = options.package_description.as_deref() {
        context.insert("package_description", desc);
    }
    if let Some(long_desc) = options.package_long_description.as_deref() {
        context.insert("package_long_description", long_desc);
    }

    let systems = if let Some(mut systems) = options.package_systems.clone() {
        if systems.is_empty() {
            systems.push("x86_64-linux".to_string());
        }
        systems
    } else {
        vec!["x86_64-linux".to_string()]
    };
    context.insert("package_systems", &systems);
    context.insert("package_license", &options.package_license);
    if let Some(upstream) = options.package_upstream.as_deref() {
        context.insert("package_upstream", upstream);
    }

    context.insert("make_desktop_file", &options.make_desktop_file);
    if let Some(exec) = options.package_executable.as_deref() {
        context.insert("package_executable", exec);
    }

    context
}

#[macro_export]
macro_rules! get_string {
    ($filename:expr) => {
        TEMPLATE
            .get_file($filename)
            .unwrap()
            .contents_utf8()
            .unwrap()
    };
}
