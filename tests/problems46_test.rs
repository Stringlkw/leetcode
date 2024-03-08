use leetcode::problem46::Solution;
#[test]
fn test_solve_problem() {
    let input = vec![1, 2, 3];
    let expected = vec![
        vec![1, 2, 3],
        vec![1, 3, 2],
        vec![2, 1, 3],
        vec![2, 3, 1],
        vec![3, 1, 2],
        vec![3, 2, 1],
    ];

    assert_eq!(Solution::permute(input), expected);
}
