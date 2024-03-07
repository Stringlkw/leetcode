use leetcode::problem77::Solution;
#[test]
fn test_solve_problem() {
    let input1 = 4;
    let intpu2 = 2;

    let expected = vec![
        vec![1, 2],
        vec![1, 3],
        vec![1, 4],
        vec![2, 3],
        vec![2, 4],
        vec![3, 4],
    ];

    assert_eq!(Solution::combine(input1,intpu2), expected);
}
