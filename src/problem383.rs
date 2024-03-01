pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut map = HashMap::new();
        magazine.chars().for_each(|c| {
            *map.entry(c).or_insert(0) += 1;
        });
        ransom_note.chars().all(|c| {
            map.get_mut(&c).map_or_else(
                || false,
                |v| {
                    if *v > 0 {
                        *v -= 1;
                        true
                    } else {
                        false
                    }
                },
            )
        })
    }
}
