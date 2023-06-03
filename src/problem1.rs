use hazs_tools::mathematics::common::merge_two_arrays_arrange_and_clean;

pub fn problem1(to: usize, number1: usize, number2: usize) -> usize {
    let mut array1: Vec<usize> = Vec::new();
    let mut array2: Vec<usize> = Vec::new();

    // These 2 for loops only solves for the number 3 and 5,
    // this is not a general solution, but a solution only to
    // euler problem 1.
    for index in 0..=(to / number1) - 1 {
        array1.push((index + 1) * number1);
    }
    for index in 0..(to / number2) - 1 {
        array2.push((index + 1) * number2);
    }

    let result = merge_two_arrays_arrange_and_clean(&mut array1, &mut array2);
    result.iter().sum()
}
