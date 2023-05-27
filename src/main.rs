#![allow(unused)]

#[cfg(debug_assertions)]
use std::env::args;

use std::time::Instant;

mod problem1;
mod problem2;
mod problem3;
mod problem4;
mod problem5;

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
        println!("");
        println!("---------------");
    }

    println!("---------------");
    println!("PROBLEM 1");
    println!("{}", problem1::problem1());
    println!("---------------");

    println!("---------------");
    println!("PROBLEM 2");
    println!("{}", problem2::problem2());
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
    println!("{}", problem5::problem5(20));
    println!("---------------");

    #[cfg(debug_assertions)]
    {
        println!("");
        println!("---------------");
        println!("FINISHED");
        println!("---------------");
    }

    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);

}
