pub struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut left = 0;
        let mut min_len = nums.len() + 1;
        for right in 0..nums.len() {
            sum += nums[right];
            while sum >= target {
                min_len = min_len.min(right - left + 1);
                sum -= nums[left];
                left += 1;
            }
        }
        if min_len == nums.len() + 1 {
            0
        } else {
            min_len as i32
        }
    }
}
