use rust_nix_templater::*;

fn main() {
    let options = Options::from_args();
    run_with_options(options, true).unwrap();
}
