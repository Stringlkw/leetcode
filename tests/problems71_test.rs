use leetcode::problem71::Solution;
#[test]
fn test_solve_problem() {
    let input = String::from("/a/./b/../../c/");

    let expected = String::from("/c");

    assert_eq!(Solution::simplify_path(input), expected);
}
