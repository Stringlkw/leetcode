use leetcode::problem169::Solution;
#[test]
fn test_solve_problem() {
    let input = vec![2,2,1,1,1,2,2];
    let expected = 2;
    assert_eq!(Solution::majority_element(input), expected);
}
