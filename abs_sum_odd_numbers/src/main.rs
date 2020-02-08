extern crate num;

use num::Integer;
use std::env;
use std::process;

fn sum_odd_numbers(n: u64) -> u64 {
    (0..)
        .take_while(|element| element < &n)
        .filter(|n| n.is_odd())
        .fold(0, |sum, element| sum + element)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("You must provide an integer as first argument of the program");
        process::exit(1);
    }
    match &args[1].parse::<u64>() {
        Ok(value) => println!(
            "Sum computed by the abstract version is {}",
            sum_odd_numbers(*value)
        ),
        Err(error) => {
            println!("Error when processing your argument: {}", error);
            process::exit(1);
        }
    }
}
