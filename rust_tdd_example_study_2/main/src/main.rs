extern crate process;

use std::env;
use process::execute;

fn main() {
    let args: Vec<String> = env::args().collect();
    execute(args);
}