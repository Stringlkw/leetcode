use leetcode::problem121::Solution;
#[test]
fn test_solve_problem() {
    let input = vec![1,2,4,2,5,7,2,4,9,0,9];

    let expected = 9;

    assert_eq!(Solution::max_profit(input), expected);
}
