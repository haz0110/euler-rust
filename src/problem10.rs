use hazs_tools::mathematics::prime_algorithms::*;
use primes::*;

pub fn problem10() -> usize {
    let mut pset = primes::Sieve::new();

    let mut storage: Vec<usize> = Vec::new();

    for (index, item) in pset.iter().enumerate().take(200_000) {
        if item >= 2_000_000 {
            break;
        }
        storage.push(item as usize);
    }
    storage.iter().sum()
}
