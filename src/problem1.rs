use hazs_tools::mathematics::common::number_of_factors_one_integer;
use hazs_tools::mathematics::common::merge_two_arrays_arrange_and_clean;

pub fn problem1() -> usize{
    let mut array1: Vec<usize> = vec![3; (1000 / 3)];
    let mut array2: Vec<usize> = vec![3; (1000 / 5)];

    let combined_array: Vec<usize> = merge_two_arrays_arrange_and_clean(&mut array1, &mut array2);

    let mut sum = 0;

    for index in 0..combined_array.len() {
        sum = sum + combined_array[index];
    }

    sum

}