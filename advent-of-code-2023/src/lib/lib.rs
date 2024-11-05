use env_logger;

pub fn logger() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
}
