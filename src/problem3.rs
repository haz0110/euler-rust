use hazs_tools::mathematics::prime_algorithms::biggest_prime_factor;

pub fn problem3(number: usize) -> usize {
    let result: usize = biggest_prime_factor(number);
    result
}
