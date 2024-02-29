pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut result = 0;
        for j in 1..prices.len() {
            let temp = prices[j] - prices[i];
            result = result.max(temp);

            if temp < 0 {
                i = j;
            }
        }
        result
    }
}
