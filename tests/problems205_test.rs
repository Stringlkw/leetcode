use leetcode::problem205::Solution;
#[test]
fn test_solve_problem() {
    let input1 = String::from("egg");
    let input2 = String::from("add");

    let expected = true;

    assert_eq!(Solution::is_isomorphic(input1, input2), expected);
}
