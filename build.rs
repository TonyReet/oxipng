use rustc_version::{version, Version};
use std::process::exit;

fn main() {
    // This should match the version in Github Actions scripts and the Readme
    const REQUIRED_VERSION: &str = "1.41.0";
    if version().unwrap() < Version::parse(REQUIRED_VERSION).unwrap() {
        eprintln!("oxipng requires rustc >= {}.", REQUIRED_VERSION);
        exit(1);
    }
}
