use leetcode::problem189::Solution;
#[test]
fn test_solve_problem() {
    let mut input1 = vec![1, 2, 3, 4, 5, 6, 7];
    let input2 = 3;
    let expected = vec![5, 6, 7, 1, 2, 3, 4];
    Solution::rotate(&mut input1, input2);
    assert_eq!(input1, expected);
}
