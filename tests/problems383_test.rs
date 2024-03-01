use leetcode::problem383::Solution;
#[test]
fn test_solve_problem() {
    let input1 = String::from("aaba");
    let input2 = String::from("bbbaaa");

    let expected = true;

    assert_eq!(Solution::can_construct(input1, input2), expected);
}
