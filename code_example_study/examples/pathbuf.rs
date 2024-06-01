use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("/usr/local/bin");
    let parts = path.into_parts();
    println!("Path: {:?}", parts);
}