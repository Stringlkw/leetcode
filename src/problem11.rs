use std::cmp::min;

pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = height.len() - 1;
        let mut max_area = 0;
        while i < j {
            max_area = max_area.max(min(height[i], height[j]) * (j - i) as i32);
            if height[i] < height[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }
        max_area
    }
}
