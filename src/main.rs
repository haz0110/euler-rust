#![allow(unused)]

#[cfg(debug_assertions)]
use std::env::args;

use std::time::Instant;

use hazs_tools::euler_specific;
use hazs_tools::euler_specific::*;
use hazs_tools::mathematics::*;

mod problem1;
mod problem10;
mod problem12;
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
    println!("{}", problem1::problem1(1000, 3, 5));
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
    let result = palindrome::palindromes(3);
    println!("{}", result[result.len() - 1]);

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

    println!("---------------");
    println!("PROBLEM 9");
    println!(
        "{}",
        euler_specific::p9_special_pythagorean_triplet::special_pythagorean_triplet()
    );
    println!("---------------");

    println!("---------------");
    println!("PROBLEM 10");
    println!("{}", problem10::problem10());
    println!("---------------");

    println!("---------------");
    println!("PROBLEM 12");
    println!("Current algorithm is too slow, so it is commmented out for now.");
    // println!("{}", problem12::problem12());
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
