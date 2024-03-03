use leetcode::problem55::Solution;
#[test]
fn test_solve_problem() {
    let input = vec![3,2,1,0,4];
    let expected = false;
    assert_eq!(Solution::can_jump(input), expected);
}
