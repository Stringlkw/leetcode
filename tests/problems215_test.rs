use leetcode::problem215::Solution;
#[test]
fn test_solve_problem() {
    let input1 = vec![3,2,1,5,6,4];
    let input2 = 2;
    let expected = 5;
    assert_eq!(Solution::find_kth_largest(input1, input2), expected);
}
