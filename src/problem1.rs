pub struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map: HashMap<i32, usize> = HashMap::new();
        let mut result = vec![0; 2];
        for (index, &num) in nums.iter().enumerate() {
            let key = target - num;

            if map.contains_key(&key) {
                result[0] = index as i32;
                result[1] = *map.get(&key).unwrap() as i32;
                break;
            } else {
                map.insert(num, index);
            }
        }
        result
    }
}
