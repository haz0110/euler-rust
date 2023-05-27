use hazs_tools::mathematics::square_operations::{square_of_sum_of_numbers, sum_of_squares};

pub fn problem6() -> usize{
    let result: usize = square_of_sum_of_numbers(100) - sum_of_squares(100);
    result
}