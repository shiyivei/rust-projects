#[macro_use]
extern crate serde_derive;

use std::io;
use std::process;
use std::io::Write;

mod blockchain;
fn main() {

    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    println!("inout miner address: ");
    io::stdout().flush();
    io::stdin().read_line(&mut miner_addr);

    println!
    
}
