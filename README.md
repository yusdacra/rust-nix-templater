# rust-nix-templater
Generates Nix files for Rust applications which uses [naersk](https://github.com/nmattia/naersk).

## Usage
```
rust-nix-templater 0.1.0

USAGE:
    rust-nix-templater [FLAGS] [OPTIONS] --license <package-license> --name <package-name>

FLAGS:
        --help                  Prints help information
    -M, --mk-desktop-file       Create a desktop file
    -T, --use-toolchain-file    Use the `rust-toolchain` file instead of a channel
    -V, --version               Prints version information

OPTIONS:
        --cachix-name <cachix-name>                      Cachix cache name
        --cachix-public-key <cachix-public-key>          Cachix cache public key
    -c, --ci <ci>...                                     Which CI systems to create CI files for. [example: -c github]
    -o, --out-dir <out-dir>
            The output dir where rendered files will be put in [default: out]

    -d, --description <package-description>              A short, single line description of the package
    -e, --executable <package-executable>
            Name of the executable `cargo build` generates. Required if your package's executable name is different from
            your package's name
    -h, --homepage <package-homepage>
            Homepage of the package. [example: -h "https://gitlab.com/example/example"]

        --icon <package-icon>
            Icon to use in the generated desktop file. [example: --icon assets/icon.ico]

    -l, --license <package-license>
            License of the package. Can be any of the values listed in
            https://github.com/NixOS/nixpkgs/blob/master/lib/licenses.nix. [example: -l mit]
    -D, --long-description <package-long-description>    A longer description of the package
    -n, --name <package-name>                            Name of the package. [example: -n icy_matrix]
    -s, --systems <package-systems>...
            Systems that the package is supported on. [example: -s x86_64-linux x86_64-darwin] Defaults to nixpkgs's
            default systems
        --xdg-categories <package-xdg-categories>...
            Categories to put in the generated desktop file. [example: --xdg-categories Network InstantMessaging]

        --xdg-comment <package-xdg-comment>
            Comment to put in the generated desktop file. Defaults to package description

        --xdg-desktop-name <package-xdg-desktop-name>
            Desktop name to put in the generated desktop file. Defaults to package name. [example: --xdg-desktop-name
            "Icy Matrix"]
        --xdg-generic-name <package-xdg-generic-name>
            Generic name to put in the generated desktop file. [example: --xdg-generic-name "Matrix Client"]

    -t, --toolchain <rust-toolchain-channel>
            Rust toolchain channel to use. [example: -t nightly] [default: stable]
```