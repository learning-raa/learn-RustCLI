#![allow(non_snake_case)]

use clap::Parser;


fn main() {
    println!("parser intro");

    let args = CliArgs::parse();
    println!("{} -> {}", args.commandName, args.value);
    println!("{} -> {}", args.uflA, args.wflBCDe);
    println!("mOtp -> {}", args.mOptCam );
}

#[derive(Parser)]
#[command(about)]
struct CliArgs {
    //#[arg(short,long)]
    commandName: String,
    #[arg(short,long, default_value_t = 0)]
    value: u8,
    #[arg(short,long, default_value_t = false)]
    uflA: bool,
    #[arg(short,long, default_value_t = false)]
    wflBCDe: bool,
    #[arg(short,long, default_value_t = false)]
    mOptCam: bool,
}

