use std::{env::{self, Args}, process};

use command_line_calculator::{quocient, product, difference, sum};

fn main() {
    let args = env::args().into_iter();
    let config= set_config(args);

    if config.op == "sum" {println!("total: {}", sum(config.values))} 
    else if config.op == "difference" {println!("total: {}", difference(config.values))} 
    else if config.op == "product" {println!("total: {}", product(config.values))} 
    else if config.op == "quocient" {println!("total: {}", quocient(config.values))} 
    else {
        println!(
            "Invalid operation! please, type cargo run help for 
            more information"
        );

        process::exit(1);
    };

}

struct Config<T> where T: Iterator<Item = f64> {
    op: String, // op = operation
    values: T, 
}

fn set_config(mut args: Args) -> Config<impl Iterator<Item = f64>> {
    args.next();
    
    let op: String = match args.next() {
        Some(op) => op.to_lowercase(),
        None => {
            eprintln!("Please, specify a match operation");
            process::exit(1);
        }
    };

    let values = args.map(|n: String| {
        let inter: Result<f64, std::num::ParseFloatError> = n.parse::<f64>();

        match inter {
            Ok(i) => i,
            Err(e) => panic!("{e:?}"),
        }
    });

    return Config { op, values };
}