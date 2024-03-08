use leetcode::problem155::MinStack;
#[test]
fn test_solve_problem() {
    let mut min_stack = MinStack::new();
    min_stack.push(-2);
    min_stack.push(0);
    min_stack.push(-3);
    assert_eq!(min_stack.get_min(), -3);
    min_stack.pop();
    assert_eq!(min_stack.top(), 0);
    assert_eq!(min_stack.get_min(), -2);
}
