pub struct Solution;
impl Solution {
    fn backtrack(
        nums: &Vec<i32>,
        used: &mut Vec<bool>,
        combination: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if combination.len() == nums.len() {
            result.push(combination.clone());
            return;
        }
        for (i, &num) in nums.iter().enumerate() {
            if used[i] == false {
                combination.push(num);
                used[i] = true;
                Self::backtrack(nums, used, combination, result);
                used[i] = false;
                combination.pop();
            }
        }
    }
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut combination: Vec<i32> = Vec::new();
        let mut used: Vec<bool> = vec![false; nums.len()];
        Self::backtrack(&nums, &mut used, &mut combination, &mut result);

        result
    }
}
