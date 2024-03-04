use leetcode::problem392::Solution;
#[test]
fn test_solve_problem() {
    let input1 = String::from("abc");
    let input2 = String::from("ahbgdc");

    let expected = true;

    assert_eq!(Solution::is_subsequence(input1, input2), expected);
}
