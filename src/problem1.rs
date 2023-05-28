use hazs_tools::mathematics::common::merge_two_arrays_arrange_and_clean;
use std::iter::Sum;

pub fn problem1() -> usize {
    let combined_array: Vec<usize> =
        merge_two_arrays_arrange_and_clean(&mut vec![3; (1000 / 3)], &mut vec![5; (1000 / 5)]);
    Sum::sum(combined_array.iter())
}
