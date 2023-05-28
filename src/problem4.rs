use hazs_tools::mathematics::palindrome::palindromes_3_digits;

pub fn problem4() -> usize {
    let result = palindromes_3_digits();
    result[result.len() - 1]
}
