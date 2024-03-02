pub struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut left = 0;
        let mut max_len = 0;
        let s: Vec<char> = s.chars().collect();
        for right in 0..s.len() {
            while s[left..right].contains(&s[right]) {
                left += 1;
            }
            max_len = max_len.max(right - left + 1);
        }
        max_len as i32
    }
}
