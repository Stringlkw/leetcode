use leetcode::problem35::Solution;
#[test]
fn test_solve_problem() {
    let input1 = vec![1, 3, 5, 6];
    let input2 = 5;
    let expected = 2;
    assert_eq!(Solution::search_insert(input1, input2), expected);
}
