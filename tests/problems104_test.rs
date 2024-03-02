use std::{cell::RefCell, rc::Rc};

use leetcode::problem104::{Solution, TreeNode};
#[test]
fn test_solve_problem() {
    let input = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 15,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
        }))),
    })));

    let expected = 3;

    assert_eq!(Solution::max_depth(input), expected);
}
