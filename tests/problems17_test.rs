use leetcode::problem17::Solution;
#[test]
fn test_solve_problem() {
    let input = String::from("23");

    let expected = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"];

    assert_eq!(Solution::letter_combinations(input), expected);
}
