use leetcode::problem209::Solution;
#[test]
fn test_solve_problem() {
    let input1 = 7;
    let input2 = vec![7, 1, 5, 3, 6, 4];

    let expected = 2;

    assert_eq!(Solution::min_sub_array_len(input1, input2), expected);
}
