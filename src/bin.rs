extern crate rot13;
use rot13::rot13;

use std::io::{self, BufRead};

/// Encodes the input from STDIN in rot13 and prints the output to STDOUT
fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", rot13(&line.unwrap()));
    }
}
