use leetcode::problem1::Solution;
#[test]
fn test_solve_problem() {
    let input1 = vec![2, 7, 11, 15];
    let input2 = 9;

    let expected = vec![0, 1];

    assert_eq!(Solution::two_sum(input1, input2), expected);
}
