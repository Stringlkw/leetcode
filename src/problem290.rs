pub struct Solution;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        use std::collections::HashMap;
        let words: Vec<&str> = s.split_whitespace().collect();
        if pattern.len() != words.len() {
            return false;
        }
        let mut map_char_word = HashMap::new();
        let mut map_word_char = HashMap::new();
        for (ch, word) in pattern.chars().zip(words) {
            map_char_word.entry(ch).or_insert(word);
            map_word_char.entry(word).or_insert(ch);
            if map_char_word.get(&ch) != Some(&word) || map_word_char.get(word) != Some(&ch) {
                return false;
            }
        }
        true
    }
}
