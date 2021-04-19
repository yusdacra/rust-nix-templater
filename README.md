# rust-nix-templater

Generates Rust projects which use [nix-cargo-integration](https://github.com/yusdacra/nix-cargo-integration).

## Features

- One place for configuration; Cargo.toml
- Generate for applications or libraries
- Support for both flakes and legacy nix
- Generates release, debug and test packages
- Cachix support
- Generates development shell (uses [devshell], supports top-level `devshell.toml` file)
- Desktop file generation
- CI file generation (GitHub Actions and GitLab CI)
- Creates Cargo project if one is not found in output directory

## Installation

- Flakes: `nix profile install github:yusdacra/rust-nix-templater`
    - Or run without installing: `nix run github:yusdacra/rust-nix-templater`
- Legacy: `nix-env -i -f "https://github.com/yusdacra/rust-nix-templater/tarball/master"`

## Examples

Simple:

```ShellSession
rust-nix-templater -l mit -n example
# is equal to
rust-nix-templater --license mit --name example
```

This will generate files in the current directory, with license set to `mit` and package name set to `example`. It will generate both build and development environment files that have a binary package, using Rust's `stable` toolchain. If the current directory doesn't already have a Cargo project, this will create one.

For a project that uses `rust-toolchain` file:

```ShellSession
rust-nix-templater -T -l mit -n example
# is equal to
rust-nix-templater --use-toolchain-file -l mit -n example
```

This will do what the previous examples does plus use `rust-toolchain` file instead of Rust's `stable` toolchain.

For a project that uses `rust-toolchain` file, but is only a library:

```ShellSession
rust-nix-templater -LT -l mit -n example
# is equal to
rust-nix-templater --library -T -l mit -n example
```

This will do what the previous example does but it won't generate a binary package (which means it also won't generate a Flake application).

For a project that uses `beta` toolchain and is hosted on GitHub:

```ShellSession
rust-nix-templater -c github -t beta -l mit -n example
# is equal to
rust-nix-templater --ci github --toolchain beta -l mit -n example
```

This will do what the first example does, but use `beta` toolchain and also generate a GitHub Actions workflow.

For more options please check `rust-nix-templater --help`.

## Usage

```
rust-nix-templater 0.2.2
Generates Nix files for Rust projects which uses naersk

USAGE:
    rust-nix-templater [FLAGS] [OPTIONS]

FLAGS:
    -A, --no-app                Whether to disable app output for flake
        --disable-build         Disable app / builds flake output generation
        --help                  Prints help information
    -L, --library               Whether to copy libraries to package output
    -T, --use-toolchain-file    Use the `rust-toolchain` file instead of a channel
    -V, --version               Prints version information

OPTIONS:
        --cachix-name <cachix-name>                      Cachix cache name. [example: --cachix-name rust-nix-templater]
        --cachix-public-key <cachix-public-key>
            Cachix cache public key. [example: --cachix-public-key "rust-nix-templater.cachix.org-
            1:Tmy1V0KK+nxzg0XFePL/++t4JRKAw5tvr+FNfHz7mIY=""]
    -c, --ci <ci>...                                     Which CI systems to create CI files for. [example: -c github]
    -o, --out-dir <out-dir>
            Output directory where generated files will be put in. [example: -o example] [default: .]

    -d, --description <package-description>              A short, single line description of the package
    -e, --executable <package-executable>
            Name of the executable `cargo build` generates. Required if your package's executable name is different from
            your package's name
    -h, --homepage <package-homepage>
            Homepage of the package. [example: -h "https://gitlab.com/example/example"]

        --icon <package-icon>
            Icon to use in the generated desktop file. [example: --icon assets/icon.ico]

    -l, --license <package-license>
            License of the package. Must be a valid Cargo.toml license. [example: mit]

    -D, --long-description <package-long-description>    A longer description of the package
    -n, --name <package-name>
            Name of the package. Must be passed when also creating a Cargo project. [example: icy_matrix]

    -s, --systems <package-systems>...
            Systems that the package is supported on. [example: -s x86_64-linux x86_64-darwin] [default: nixpkgs default
            systems]
        --xdg-categories <package-xdg-categories>...
            Categories to put in the generated desktop file. [example: --xdg-categories Network InstantMessaging]

        --xdg-comment <package-xdg-comment>
            Comment to put in the generated desktop file. [default: package description]

        --xdg-desktop-name <package-xdg-desktop-name>
            Desktop name to put in the generated desktop file. Defaults to package name. [example: --xdg-desktop-name
            "Icy Matrix"]
        --xdg-generic-name <package-xdg-generic-name>
            Generic name to put in the generated desktop file. [example: --xdg-generic-name "Matrix Client"]

    -t, --toolchain <rust-toolchain-channel>
            Rust toolchain channel to use. [example: -t nightly] [default: stable]
```

## Development

### Building

The Nix flake provides two main build types: release and debug, akin to the Cargo build types.

For debugging, build with debug symbols, but don't test:

```ShellSession
nix build .#rust-nix-templater-debug
```

For release, which compiles in Cargo release mode:

```ShellSession
nix build .#rust-nix-templater
# or
nix build
```

### Testing

With Nix flakes, you can check the flake.nix format and run the tests with:

```ShellSession
nix flake check
```

This will build and test the `rust-nix-templater-tests` attribute (which also runs Cargo tests).

[devshell]: https://github.com/numtide/devshell "devshell"
