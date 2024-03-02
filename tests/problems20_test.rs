use leetcode::problem20::Solution;
#[test]
fn test_solve_problem() {
    let input = String::from("()");

    let expected = true;

    assert_eq!(Solution::is_valid(input), expected);
}
