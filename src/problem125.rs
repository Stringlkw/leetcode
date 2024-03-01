pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s: String = s.chars()
                         .filter(|c| c.is_alphanumeric())
                         .map(|c| c.to_ascii_lowercase())
                         .collect();
        s == s.chars().rev().collect::<String>()
    }
}