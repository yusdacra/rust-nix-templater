use crate::{make_test, options::Options, run_with_options};
use std::{
    ops::Not,
    path::{Path, PathBuf},
};

fn pass(_: &Path) -> bool {
    true
}

#[macro_export]
macro_rules! make_test {
    {
        $(#[$meta:meta])*
        name: $tname:ident;
        $( $name:ident: $value:expr, )*
    } => {
        make_test! {
            $(#[$meta])*
            name: $tname;
            test: pass;
            $( $name: $value, )*
        }
    };
    {
        $(#[$meta:meta])*
        name: $tname:ident;
        test: $test_fn:ident;
        $( $name:ident: $value:expr, )*
    } => {
        $(#[$meta])*
        #[test]
        fn $tname() {
            let out_dir = PathBuf::from(
                format!(
                    "/tmp/rust-nix-templater-{}-test",
                    std::time::UNIX_EPOCH.elapsed().unwrap().as_nanos()
                )
            );
            let result = run_with_options(
                Options {
                    $( $name: $value, )*
                    out_dir: out_dir.clone(),
                    package_name: Some("test".to_owned()),
                    ..Options::default()
                },
                false,
            );
            match result {
                Ok(_) => {
                    assert!($test_fn (out_dir.as_path()));
                    drop(std::fs::remove_dir_all(out_dir));
                }
                Err(err) => {
                    drop(std::fs::remove_dir_all(out_dir));
                    panic!("backtrace:\n{}", err.backtrace());
                }
            }
        }
    };
}

fn check_flake(out: &Path) -> bool {
    for path in super::BASE_FILES {
        let path = path.eq("gitignore").then(|| ".gitignore").unwrap_or(path);
        if out.join(path).exists().not() {
            eprintln!("'{}' doesn't exist in written files", path);
            return false;
        }
    }
    true
}

make_test! {
    name: simple;
    test: check_flake;
}

make_test! {
    name: with_desktop_file;
    package_xdg_desktop_name: Some("name".to_string()),
}

fn check_github(out: &Path) -> bool {
    out.join(".github/workflows/nix.yml").exists()
}

make_test! {
    name: with_github_ci;
    test: check_github;
    github_ci: true,
}

fn check_gitlab(out: &Path) -> bool {
    out.join(".gitlab-ci.yml").exists()
}

make_test! {
    name: with_gitlab_ci;
    test: check_gitlab;
    gitlab_ci: true,
}

make_test! {
    name: with_cachix;
    cachix_name: Some(String::from("test")),
    cachix_public_key: Some(String::from("test")),
}

make_test! {
    name: cachix_no_key;
    cachix_name: Some(String::from("test")),
}
