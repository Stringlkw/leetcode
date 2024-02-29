use leetcode::problem80::Solution;
#[test]
fn test_solve_problem() {
    let mut input = vec![0,0,0,1,1,1,1,2,3,3];
    let expected = 7;
    assert_eq!(Solution::remove_duplicates(&mut input), expected);
}
