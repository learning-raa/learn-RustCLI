#![allow(non_snake_case)]

use clap::Parser;

use std::env;

fn main() {
    println!("Hello, cli world!");

    for item in env::args() {
        println!("item: {}", item);
    }

    println!("parser intro");

    let args = FirstCLI::parse();
    println!("{} -> {}", args.command, args. value);
}


#[derive(Parser)]
struct FirstCLI {
    command: String,
    value: i64,
}

