pub struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        use std::collections::HashMap;
        let mut map_s_t = HashMap::new();
        let mut map_t_s = HashMap::new();
        for (c_s, c_t) in s.chars().zip(t.chars()) {
            let entry_s_t = map_s_t.entry(c_s).or_insert(c_t);
            let entry_t_s = map_t_s.entry(c_t).or_insert(c_s);
            if *entry_s_t != c_t || *entry_t_s != c_s {
                return false;
            }
        }
        true
    }
}
