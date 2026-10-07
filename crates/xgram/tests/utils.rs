#![allow(
    unused,
    reason = "Initialized in all test files separately, causes false warning"
)]

pub fn before_test() {
    dotenvy::from_filename("tests.env").expect("failed to load .env");
    pretty_env_logger::init();
}

pub fn get_token() -> String {
    std::env::var("TOKEN").expect("TOKEN environment variable is not set")
}
