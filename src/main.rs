use std::{env::{self, Args}, process};
use command_line_calculator::{quocient, product, difference, sum};

fn main() {
    let config= Config::build(env::args()).unwrap_or_else(|e| {
        eprint!("{}", e);
        process::exit(1);
    });

    match config.op.as_str() {
        "sum" => println!("total: {}", sum(config.values.into_iter())),
        "difference" => println!("total: {}", difference(config.values.into_iter())),
        "product" => println!("total: {}", product(config.values.into_iter())),
        "quocient" => println!("total: {}", quocient(config.values.into_iter())),
        _ => {
            eprintln!(
                "Invalid Operation: {}. \nTry: | difference | product | quocient | sum |", 
                config.op
            );
            process::exit(1);
        }
    }
}

struct Config {
    op: String, // op = operation
    values: Vec<f64>, 
}

impl Config {
    fn build(mut args: Args) -> Result<Config, &'static str>  {
        args.next();

        let op = match args.next() {
            Some(op) => op.to_lowercase(),
            None => {
                eprintln!("Invalid Operation");
                process::exit(1); 
            }
        };

        let mut values: Vec<f64> = Vec::new();

        for v in args {
            let value: Result<f64, &'static str> = v.parse().map_err(|_| "Parsing failed"); 
           
            match value {
                Ok(n) => values.push(n),
                Err(e) => return Err(e)
            }
        }

        return Ok(Config {op, values})
    }
}
