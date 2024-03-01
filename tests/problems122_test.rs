use leetcode::problem122::Solution;
#[test]
fn test_solve_problem() {
    let input = vec![7,1,5,3,6,4];

    let expected = 7;

    assert_eq!(Solution::max_profit(input), expected);
}
