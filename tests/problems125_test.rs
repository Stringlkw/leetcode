use leetcode::problem125::Solution;
#[test]
fn test_solve_problem() {
    let input = String::from("A man, a plan, a canal: Panama");

    let expected = true;

    assert_eq!(Solution::is_palindrome(input), expected);
}
