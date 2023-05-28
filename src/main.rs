#![allow(unused)]

#[cfg(debug_assertions)]
use std::env::args;

use std::time::Instant;

use hazs_tools::euler_specific::*;
use hazs_tools::mathematics::*;

mod problem1;
mod problem3;
mod problem4;

fn main() {
    let now = Instant::now();

    #[cfg(debug_assertions)]
    {
        println!("---------------");
        println!("TARGET");
        let args: Vec<String> = args().collect();
        for argument in args.iter() {
            print!("{} ", argument);
        }
        println!();
        println!("---------------");
    }

    println!("---------------");
    println!("PROBLEM 1");
    println!("{}", problem1::problem1());
    println!("---------------");

    println!("---------------");
    println!("PROBLEM 2");
    println!(
        "{}",
        common::sum_of_even_array_items(&mut fibonacci::fibonacci_series(4_000_000))
    );
    println!("---------------");

    println!("---------------");
    println!("PROBLEM 3");
    println!("{}", problem3::problem3(600_851_475_143));
    println!("---------------");

    println!("---------------");
    println!("PROBLEM 4");
    println!("{}", problem4::problem4());
    println!("---------------");

    println!("---------------");
    println!("PROBLEM 5");
    println!(
        "{}",
        p5_smallest_multiple::smallest_multiple(20, 1_000_000_000)
    );
    println!("---------------");

    println!("---------------");
    println!("PROBLEM 6");
    println!(
        "{}",
        square_operations::square_of_sum_of_numbers(100) - square_operations::sum_of_squares(100)
    );
    println!("---------------");

    println!("---------------");
    println!("PROBLEM 7");
    println!("{}", prime_algorithms::nth_prime(10_001));
    println!("---------------");

    #[cfg(debug_assertions)]
    {
        println!("\n");
        println!("---------------");
        println!("FINISHED");
        println!("---------------");
    }

    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
}
