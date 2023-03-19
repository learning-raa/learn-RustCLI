#![allow(non_snake_case)]

use std::env;

fn main() {
    println!("Hello, world!");

    for item in env::args() {
        println!("item: {}", item);
    }
}
