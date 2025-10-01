use std::{env::{self, Args}, process};
use command_line_calculator::{quocient, product, difference, sum};

fn main() {
    let args = env::args().into_iter();
    let config= set_config(args);

    match config.op.as_str() {
        "sum" => println!("total: {}", sum(config.values)),
        "difference" => println!("total: {}", difference(config.values)),
        "product" => println!("total: {}", product(config.values)),
        "quocient" => println!("total: {}", quocient(config.values)),
        _ => {
            eprintln!(
                "Invalid Operation: {}. \nTry: | difference | product | quocient | sum |", 
                config.op
            );
            process::exit(1);
        }
    }
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