use std::{env, process};

use command_liner_caculator::{quocient, product, difference, sum};

struct Config<T> where T: Iterator<Item = f64> {
    op: String, // op = operation
    values: T, 
}

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

fn set_config<T>(mut args: T) -> Config<impl Iterator<Item = f64>> 
where T: Iterator<Item = String> {
    args.next();
    
    let op: String = args.next().unwrap().to_lowercase();
    let values = args.map(|n: String| {
        let inter: Result<f64, std::num::ParseFloatError> = n.parse::<f64>();

        match inter {
            Ok(i) => i,
            Err(e) => panic!("{e:?}"),
        }
    });

    return Config { op, values };
}