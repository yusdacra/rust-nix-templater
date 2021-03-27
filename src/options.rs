use std::{
    fmt::{self, Display, Formatter},
    path::PathBuf,
    str::FromStr,
};

use structopt::StructOpt;

/// Generates Nix files for Rust projects which uses naersk.
#[allow(clippy::struct_excessive_bools)]
#[derive(StructOpt, Debug, Default)]
#[structopt(name = "rust-nix-templater")]
pub(crate) struct Options {
    /// Create a desktop file.
    #[structopt(short = "M", long = "mk-desktop-file")]
    pub(crate) make_desktop_file: bool,
    /// Which CI systems to create CI files for. [example: -c github]
    #[structopt(short, long)]
    pub(crate) ci: Vec<CiType>,
    /// Disable build files generation.
    #[structopt(long)]
    pub(crate) disable_build: bool,

    /// Output dir where rendered files will be put in. [example: -o .]
    #[structopt(short, long, default_value = "out")]
    pub(crate) out_dir: PathBuf,

    /// License of the package. Can be any of the values listed in https://github.com/NixOS/nixpkgs/blob/master/lib/licenses.nix. [example: mit]
    #[structopt(parse(from_str))]
    pub(crate) package_license: String,
    /// Name of the package. [example: icy_matrix]
    #[structopt(parse(from_str))]
    pub(crate) package_name: String,
    /// Systems that the package is supported on. [example: -s x86_64-linux x86_64-darwin] [default: nixpkgs default systems]
    #[structopt(short = "s", long = "systems")]
    pub(crate) package_systems: Option<Vec<String>>,

    /// A short, single line description of the package.
    #[structopt(short = "d", long = "description")]
    pub(crate) package_description: Option<String>,
    /// A longer description of the package.
    #[structopt(short = "D", long = "long-description")]
    pub(crate) package_long_description: Option<String>,
    /// Homepage of the package. [example: -h "https://gitlab.com/example/example"]
    #[structopt(short = "h", long = "homepage")]
    pub(crate) package_homepage: Option<String>,

    /// Create a library package instead of a binary package.
    #[structopt(short = "L", long = "library")]
    pub(crate) package_lib: bool,
    /// Name of the executable `cargo build` generates.
    /// Required if your package's executable name is different from your package's name.
    #[structopt(short = "e", long = "executable")]
    pub(crate) package_executable: Option<String>,

    /// Icon to use in the generated desktop file. [example: --icon assets/icon.ico]
    #[structopt(long = "icon")]
    pub(crate) package_icon: Option<String>,
    /// Comment to put in the generated desktop file. [default: package description]
    #[structopt(long = "xdg-comment")]
    pub(crate) package_xdg_comment: Option<String>,
    /// Desktop name to put in the generated desktop file. Defaults to package name. [example: --xdg-desktop-name "Icy Matrix"]
    #[structopt(long = "xdg-desktop-name")]
    pub(crate) package_xdg_desktop_name: Option<String>,
    /// Generic name to put in the generated desktop file. [example: --xdg-generic-name "Matrix Client"]
    #[structopt(long = "xdg-generic-name")]
    pub(crate) package_xdg_generic_name: Option<String>,
    /// Categories to put in the generated desktop file. [example: --xdg-categories Network InstantMessaging]
    #[structopt(long = "xdg-categories")]
    pub(crate) package_xdg_categories: Option<Vec<String>>,

    /// Use the `rust-toolchain` file instead of a channel.
    #[structopt(short = "T", long = "use-toolchain-file")]
    pub(crate) rust_toolchain_file: bool,
    /// Rust toolchain channel to use. [example: -t nightly]
    #[structopt(short = "t", long = "toolchain", default_value = "stable")]
    pub(crate) rust_toolchain_channel: RustToolchainChannel,

    /// Cachix cache name. [example: --cachix-name rust-nix-templater]
    #[structopt(long)]
    pub(crate) cachix_name: Option<String>,
    /// Cachix cache public key. [example: --cachix-public-key "rust-nix-templater.cachix.org-1:Tmy1V0KK+nxzg0XFePL/++t4JRKAw5tvr+FNfHz7mIY=""]
    #[structopt(long)]
    pub(crate) cachix_public_key: Option<String>,
}

#[derive(Debug)]
pub(crate) enum CiTypeParseError {
    NotFound,
}

impl Display for CiTypeParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::NotFound => write!(
                f,
                "No such CI system supported. Supported CI systems are 'github'."
            ),
        }
    }
}

#[derive(Debug)]
pub(crate) enum CiType {
    Github,
}

impl FromStr for CiType {
    type Err = CiTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().trim() {
            "github" => Ok(Self::Github),
            _ => Err(Self::Err::NotFound),
        }
    }
}

#[derive(Debug)]
pub(crate) enum RustToolchainChannelParseError {
    Invalid,
}

impl Display for RustToolchainChannelParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Invalid => write!(f, "Invalid channel name specified. Valid channels are 'stable', 'beta' and 'nightly'."),
        }
    }
}

#[derive(Debug)]
pub(crate) enum RustToolchainChannel {
    Stable,
    Beta,
    Nightly,
}

impl Default for RustToolchainChannel {
    fn default() -> Self {
        Self::Stable
    }
}

impl FromStr for RustToolchainChannel {
    type Err = RustToolchainChannelParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().trim() {
            "stable" => Ok(Self::Stable),
            "beta" => Ok(Self::Beta),
            "nightly" => Ok(Self::Nightly),
            _ => Err(Self::Err::Invalid),
        }
    }
}

impl Display for RustToolchainChannel {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Stable => write!(f, "stable"),
            Self::Beta => write!(f, "beta"),
            Self::Nightly => write!(f, "nightly"),
        }
    }
}
