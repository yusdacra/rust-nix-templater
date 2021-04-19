use rust_nix_templater::*;

fn main() {
    let options = Options::from_args();
    std::process::exit(run_with_options(options, true).unwrap());
}
