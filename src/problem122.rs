pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        for i in 1..prices.len() {
            let profit = prices[i] - prices[i - 1];
            if profit > 0 {
                max_profit += profit;
            }
        }
        max_profit
    }
}
