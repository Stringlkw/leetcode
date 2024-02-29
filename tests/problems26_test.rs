use leetcode::problem26;
#[test]
fn test_solve_problem() {
    let mut input = vec![1, 1, 2];
    let expected = 2;
    assert_eq!(problem26::Solution::remove_duplicates(&mut input), expected);
}
