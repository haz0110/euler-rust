#![allow(unused)]

#[cfg(debug_assertions)]
use std::env::args;

use std::time::Instant;
use hazs_tools::mathematics;

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
    {
        let mut result: Vec<usize> = Vec::new();
        for number in 1..1000 {
            if number % 3 == 0 || number % 5 == 0 {
                result.push(number);
            }
        }
        println!("Result is: {}", mathematics::common::sum_of_array_items(result));
    }
    println!("---------------");

    println!("---------------");
    println!("PROBLEM 2");
    {
        let mut fibonacci_numbers = mathematics::sequences::fibonacci(4_000_000);
        println!("Result is: {}", mathematics::common::sum_of_even_array_items(&mut fibonacci_numbers));
    }
    println!("---------------");

    println!("---------------");
    println!("PROBLEM 3");
    {
        let number: usize = 1_851_475_143;
        let mut prime_factors = mathematics::common::prime_factors(number);
        println!("Result is: {}", prime_factors[prime_factors.len() - 1]);
    }
    println!("---------------");

    println!("---------------");
    println!("PROBLEM 4");
    println!("---------------");

    println!("---------------");
    println!("PROBLEM 5");
    println!("---------------");

    println!("---------------");
    println!("PROBLEM 6");
    println!("---------------");

    println!("---------------");
    println!("PROBLEM 7");
    println!("---------------");

    println!("---------------");
    println!("PROBLEM 9");
    println!(
        "{}", 1
    );
    println!("---------------");

    println!("---------------");
    println!("PROBLEM 10");
    println!("---------------");

    println!("---------------");
    println!("PROBLEM 12");
    println!("---------------");

    // #[cfg(debug_assertions)]
    {
        println!("\n");
        println!("---------------");
        println!("FINISHED");
        println!("---------------");
    }

    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
}
