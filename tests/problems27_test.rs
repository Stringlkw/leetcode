use leetcode::problem27;
#[test]
fn test_solve_problem() {
    let mut input1 = vec![3, 2, 2, 3];
    let input2 = 3;
    let expected = 2;
    assert_eq!(
        problem27::Solution::remove_element(&mut input1, input2),
        expected
    );
}
