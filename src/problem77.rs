

pub struct Solution;
impl Solution {
    fn backtrack(
        start_index: usize,
        k: usize,
        n: usize,
        combination: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if combination.len() == k {
            result.push(combination.clone());
            return;
        }
        for i in start_index..=n {
            if k - combination.len() > n - i + 1 {
                break;
            }
            combination.push(i as i32);
            Self::backtrack(i+ 1, k, n, combination, result);
            combination.pop();
        }
    }
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut combination: Vec<i32> = Vec::new();
        Self::backtrack(1, k as usize, n as usize, &mut combination, &mut result);
        result
    }
}
