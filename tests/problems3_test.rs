use leetcode::problem3::Solution;
#[test]
fn test_solve_problem() {
    let input = String::from("abcabcbb");


    let expected = 3;

    assert_eq!(Solution::length_of_longest_substring(input), expected);
}
