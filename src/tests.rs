use crate::{
    make_test,
    options::{CiType, Options, RustToolchainChannel},
    run_with_options,
};
use std::{path::PathBuf, str::FromStr};

#[macro_export]
macro_rules! make_test {
    {
        $(#[$meta:meta])*
        $tname:ident;
        $( $name:ident: $value:expr, )*
    } => {
        $(#[$meta])*
        #[test]
        fn $tname() {
            let out_dir = PathBuf::from(format!("/tmp/rust-nix-templater-{}-test", std::time::UNIX_EPOCH.elapsed().unwrap().as_nanos()));
            run_with_options(Options {
                $( $name: $value, )*
                out_dir: out_dir.clone(),
                package_name: Some("test".to_owned()),
                ..Options::default()
            }, false).unwrap();
            let _ = std::fs::remove_dir_all(out_dir);
        }
    };
}

make_test! {
    simple;
}

make_test! {
    with_desktop_file;
    make_desktop_file: true,
}

make_test! {
    with_github_ci;
    ci: vec![CiType::Github],
}

make_test! {
    with_cachix;
    cachix_name: Some(String::from("test")),
    cachix_public_key: Some(String::from("test")),
}

make_test! {
    with_toolchain;
    rust_toolchain_channel: RustToolchainChannel::Nightly,
}

make_test! {
    with_toolchain_file;
    rust_toolchain_file: true,
}

make_test! {
    with_executable;
    package_executable: Some(String::from("testg")),
}

make_test! {
    toolchain_file_channel;
    rust_toolchain_channel: RustToolchainChannel::Nightly,
    rust_toolchain_file: true,
}

make_test! {
    cachix_no_key;
    cachix_name: Some(String::from("test")),
}

make_test! {
    #[should_panic]
    wrong_toolchain_channel;
    rust_toolchain_channel: RustToolchainChannel::from_str("definitely invalid channel").unwrap(),
}

make_test! {
    #[should_panic]
    wrong_ci_name;
    ci: vec![CiType::from_str("invalid ci tpye").unwrap()],
}

make_test! {
    lib_package;
    package_lib: true,
}

make_test! {
    lib_package_desktop;
    package_lib: true,
    make_desktop_file: true,
}
