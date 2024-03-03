use leetcode::problem289::Solution;
#[test]
fn test_solve_problem() {
    let mut input = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];

    let expected = vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]];

    Solution::game_of_life(&mut input);
    assert_eq!(input, expected);
}
