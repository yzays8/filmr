fn main() {
    if let Err(e) = filmr::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
