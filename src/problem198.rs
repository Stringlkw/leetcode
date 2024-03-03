pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp: Vec<i32> = Vec::with_capacity(n + 1);
        dp.push(0);
        dp.push(nums[0]);
        for i in 1..n {
            dp.push(dp[i].max(dp[i - 1] + nums[i]));
        }
        dp[n]
    }
}
