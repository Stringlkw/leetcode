use leetcode::problem198::Solution;
#[test]
fn test_solve_problem() {
    let input = vec![4,1,2,9];
    let expected = 13;
    assert_eq!(Solution::rob(input), expected);
}
