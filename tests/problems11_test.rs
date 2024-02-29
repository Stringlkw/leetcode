use leetcode::problem11::Solution;
#[test]
fn test_solve_problem() {
    let input = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let expected = 49;
    assert_eq!(Solution::max_area(input), expected);
}
