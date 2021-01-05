# rust-nix-templater
Generates Nix files for Rust applications which uses [naersk](https://github.com/nmattia/naersk).

## Usage
```
rust-nix-templater 0.1.0

USAGE:
    rust-nix-templater [FLAGS] [OPTIONS] <package-name>

FLAGS:
    -h, --help               Prints help information
        --mk-desktop-file    Create a desktop file
    -V, --version            Prints version information

OPTIONS:
    -o, --out-dir <out-dir>                           The output dir where rendered files will be put in
        --pkg-desc <package-description>              A short, single line description of the package
        --pkg-exec <package-executable>
            Only required if your package's executable name is different from your package's name

        --pkg-license <package-license>
            License of the package. Can be any of the values listed in
            https://github.com/NixOS/nixpkgs/blob/master/lib/licenses.nix [default: gpl3]
        --pkg-desc-long <package-long-description>    A longer description of the package
        --pkg-sys <package-systems>...
            Systems that the package is supported on. [example: x86_64-linux,x86-linux] [default: x86_64-linux]

        --pkg-repo <package-upstream>
            Upstream repository of the package. [example: "https://gitlab.com/example/example"]


ARGS:
    <package-name>    Name of the package
```