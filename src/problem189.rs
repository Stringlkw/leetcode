pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut count = 1;
        for num in nums {
            if num == result {
                count += 1;
            } else {
                count -= 1;
                if count == 0 {
                    result = num;
                    count = 1;
                }
            }
        }
        result
    }
}
