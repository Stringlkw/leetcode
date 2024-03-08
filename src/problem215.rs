pub struct Solution;
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        
        

        let mut min_heap = BinaryHeap::<Reverse<i32>>::new();
        nums.iter().enumerate().for_each(|(i, &num)| {
            if i < k as usize {
                min_heap.push(Reverse(num));
            } else {
                if num > min_heap.peek().unwrap().0 {
                    min_heap.pop();
                    min_heap.push(Reverse(num));
                }
            }
        });
        min_heap.peek().unwrap().0
    }
}
