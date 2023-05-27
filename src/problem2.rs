use hazs_tools::mathematics::fibonacci::fibonacci_series;
use hazs_tools::mathematics::common::sum_of_even_array_items;

pub fn problem2() -> usize {
    sum_of_even_array_items(&mut fibonacci_series(4_000_000))
}