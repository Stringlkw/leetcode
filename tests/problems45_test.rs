use leetcode::problem45::Solution;
#[test]
fn test_solve_problem() {
    let input = vec![1,2,1,1,1];
    let expected = 3;
    assert_eq!(Solution::jump(input), expected);
}
