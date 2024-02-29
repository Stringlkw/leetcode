pub struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let length = nums.len() as i32;
        let k = k % length;
        let (left, right) = nums.split_at_mut((length - k) as usize);
        left.reverse();
        right.reverse();
        nums.reverse();
    }
}
