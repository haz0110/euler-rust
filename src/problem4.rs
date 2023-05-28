use hazs_tools::mathematics::palindrome::palindromes;

pub fn problem4() -> usize {
    let result = palindromes(3);
    result[result.len() - 1]
}
