use leetcode::problem70::Solution;
#[test]
fn test_solve_problem() {
    let input = 3;
    let expected = 3;
    assert_eq!(Solution::climb_stairs(input), expected);
}
