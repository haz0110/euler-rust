use hazs_tools::mathematics::common::factors;
use hazs_tools::mathematics::triangular_numbers::is_triangular;

pub fn problem12() -> usize {
    for index in 1..100_000_000 {
        if is_triangular(index) && factors(index, true, true).len() > 500 {
            println!("Number: {index}");
            for factor in factors(index, true, true).iter() {
                println!("Factor: {factor}");
            }

            return index;
        }
    }
    0
}
