use hazs_tools::mathematics::common::merge_two_arrays_arrange_and_clean;

pub fn problem1() -> usize {

    let mut array1: Vec<usize> = vec![3; 1000/3].into_iter().map(|x| x + 3).rev().collect();
    let mut array2: Vec<usize> = vec![3; 1000/3];

    merge_two_arrays_arrange_and_clean(&mut array1, &mut array2).iter().sum()
}
