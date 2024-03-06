use std::collections::HashMap;

pub struct Solution;
impl Solution {
    fn backtrack(
        digits: &[char],
        depth: usize,
        map: &HashMap<char, &str>,
        combination: &mut String,
        result: &mut Vec<String>,
    ) {
        if depth == digits.len() {
            result.push(combination.clone());
            return;
        }
        if let Some(&letters) = map.get(&digits[depth]) {
            for letter in letters.chars() {
                combination.push(letter);
                Self::backtrack(digits, depth + 1, map, combination, result);
                combination.pop();
            }
        }
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let map: HashMap<char, &str> = [
            ('2', "abc"),
            ('3', "def"),
            ('4', "ghi"),
            ('5', "jkl"),
            ('6', "mno"),
            ('7', "pqrs"),
            ('8', "tuv"),
            ('9', "wxyz"),
        ]
        .iter()
        .cloned()
        .collect();
        let digits: Vec<char> = digits.chars().collect();
        let mut result = Vec::new();
        let mut combination = String::new();

        Self::backtrack(&digits, 0, &map, &mut combination, &mut result);
        result
    }
}
