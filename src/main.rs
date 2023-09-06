fn main() {
    if let Err(e) = fixtojson::get_args().and_then(fixtojson::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
