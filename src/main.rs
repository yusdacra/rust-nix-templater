use rust_nix_templater::*;

fn main() {
    let options = Options::from_args();
    let result = run_with_options(options, true);
    if let Err(err) = result {
        panic!("backtrace:\n{}", err.backtrace());
    }
}
