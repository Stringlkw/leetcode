pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let count = nums
            .iter()
            .rev()
            .skip(1)
            .fold(0, |count, num| if *num <= count { count + 1 } else { 0 });
        count == 0
    }
}
