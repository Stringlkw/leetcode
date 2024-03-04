pub struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s = s.chars();
        let mut s_next = s.next();
        for c in t.chars() {
            if s_next == Some(c) {
                s_next = s.next();
            }
        }
        s_next.is_none()
    }
}
