extern crate num_bigint as bigint;
extern crate num_traits;

use bigint::BigUint;
use num_traits::{One, Zero};
use std::io;
use std::mem::replace;
use std::time::Instant;

fn main() {
    println!("\nFibonacci Generator\n");

    loop {
        println!("Select algorithm type:\n    iterative (I) - fast\n    recursive: (R) - slow\n");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let input = input.trim();

        match input {
            "help" => {
                println!(
                    "Select the type of algorithm to use\n    'I' for iterative - the fastest\n    'R' for recursive\n    'exit' to exit.\n"
                );
                continue;
            }
            "exit" => break,
            "I" | "i" => {
                let d = get_index('I');
                if d == 'x' {
                    break;
                }
            }
            "R" | "r" => {
                let d = get_index('R');
                if d == 'x' {
                    break;
                }
            }
            &_ => println!(
                "Sorry, I don't recognise that command, enter 'help' for a list of commands."
            ),
        }
    }
}

fn get_index(fib_type: char) -> char {
    loop {
        println!("Please enter fibonacci index:");
        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line.");

        let index = index.trim();

        match index {
            "help" => {
                println!("\n    Enter an integer greater or equal to 0,\n    'back' to go back.\n    'exit' to exit\n")
            }
            "I" | "i" => {
                println!("\nNow using iterative algorithm");
                get_index('I');
            }
            "R" | "r" => {
                println!("\nNow using recursive algorithm");
                get_index('R');
            }
            "back" => return 'b',
            "exit" => return 'x',
            &_ => match index.parse::<i64>() {
                Ok(num) => {
                    if num < 0 {
                        println!("Invalid input, index must be an integer of 0 or more.")
                    };
                    let start_time = Instant::now();
                    if fib_type == 'I' {
                        println!("Iterative: {}", fib_itr(num));
                        println!("Time: {:?}\n", start_time.elapsed().as_millis());
                    } else {
                        println!("Recursive: {}", fib_rec(num));
                        println!("Time: {:?}\n", start_time.elapsed().as_millis());
                    }
                }
                Err(_) => {
                    println!("That's not a number!\n");
                    continue;
                }
            },
        }
    }
}

fn fib_rec(n: i64) -> u64 {
    if n < 2 {
        n as u64
    } else {
        fib_rec(n - 1) + fib_rec(n - 2)
    }
}

fn fib_itr(n: i64) -> BigUint {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();

    for _ in 0..n {
        let f2 = f0 + &f1;
        f0 = replace(&mut f1, f2)
    }

    f0
}
