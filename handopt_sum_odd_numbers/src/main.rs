extern crate num;

use num::Integer;
use std::env;
use std::process;

fn sum_odd_numbers(n: u64) -> u64 {
    let mut acc = 0;
    for element in 0.. {
        if element >= n {
            break;
        }
        if element.is_odd() {
            acc += element;
        }
    }
    acc
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("You must provide an integer as first argument of the program");
        process::exit(1);
    }
    match &args[1].parse::<u64>() {
        Ok(value) => println!(
            "Sum computed by the hand writtened optimized version is {}",
            sum_odd_numbers(*value)
        ),
        Err(error) => {
            println!("Error when processing your argument: {}", error);
            process::exit(1);
        }
    }
}
