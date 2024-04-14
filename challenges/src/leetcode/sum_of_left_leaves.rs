#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;

pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let root = root.unwrap();
    let x = root.borrow();
    let mut sum = 0;
    if let Some(right) = &x.right {
        let clones = Rc::clone(right);
        sum += sum_of_left_leaves(Some(clones));
    }
    if let Some(left) = &x.left {
        let clones = Rc::clone(left);
        let left_val = left.borrow();
        sum += left_val.val + sum_of_left_leaves(Some(clones))
    }
    sum
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::leetcode::sum_of_left_leaves::TreeNode;

    #[test]
    fn one_deep() {
        use super::sum_of_left_leaves;

        let mut parent = TreeNode::new(3);
        let parent_left = TreeNode::new(9);
        let parent_right = TreeNode::new(20);
        parent.left = Some(Rc::new(RefCell::new(parent_left)));
        parent.right = Some(Rc::new(RefCell::new(parent_right)));
        assert_eq!(sum_of_left_leaves(Some(Rc::new(RefCell::new(parent)))), 9);
    }

    #[test]
    fn roots() {
        use super::sum_of_left_leaves;

        let parent = TreeNode::new(1);
        assert_eq!(sum_of_left_leaves(Some(Rc::new(RefCell::new(parent)))), 0);
    }

    #[test]
    fn two_deep() {
        use super::sum_of_left_leaves;

        let mut parent = TreeNode::new(3);
        let parent_left = TreeNode::new(9);
        let mut parent_right = TreeNode::new(20);
        let child_left = TreeNode::new(15);
        let child_right = TreeNode::new(7);
        parent_right.left = Some(Rc::new(RefCell::new(child_left)));
        parent_right.right = Some(Rc::new(RefCell::new(child_right)));
        parent.left = Some(Rc::new(RefCell::new(parent_left)));
        parent.right = Some(Rc::new(RefCell::new(parent_right)));
        assert_eq!(sum_of_left_leaves(Some(Rc::new(RefCell::new(parent)))), 24);
    }
}
