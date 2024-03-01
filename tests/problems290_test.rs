use leetcode::problem290::Solution;
#[test]
fn test_solve_problem() {
    let input1 = String::from("abba");
    let input2 = String::from("dog cat cat dog");

    let expected = true;

    assert_eq!(Solution::word_pattern(input1, input2), expected);
}
