pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i = 0;
        let mut j = numbers.len() - 1;
        while i < j {
            {
                if numbers[i] + numbers[j] > target {
                    j -= 1;
                } else if numbers[i] + numbers[j] < target {
                    i += 1;
                } else {
                    return vec![(i + 1) as i32, (j + 1) as i32];
                }
            }
        }
        vec![(i + 1) as i32, (j + 1) as i32]
    }
}
