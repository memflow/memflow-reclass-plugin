#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    // compile with default values from Cargo.toml
    winres::WindowsResource::new().compile().unwrap();
}

#[cfg(not(windows))]
fn main() {}
