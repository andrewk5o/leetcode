// 104. Maximum Depth of Binary Tree
// https://leetcode.com/problems/maximum-depth-of-binary-tree/

use std::cell::RefCell;
use std::rc::Rc;

use crate::solutions::tree_node::TreeNode;

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let left = max_depth(root.as_ref().unwrap().borrow().left.clone());
    let right = max_depth(root.as_ref().unwrap().borrow().right.clone());

    std::cmp::max(left, right) + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        assert_eq!(max_depth(None), 0);
    }

    #[test]
    fn test_single_node() {
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        assert_eq!(max_depth(Some(root)), 1);
    }

    #[test]
    fn test_balanced_tree() {
        // Create a tree like:
        //     3
        //    / \
        //   9  20
        //      / \
        //     15  7
        let root = Rc::new(RefCell::new(TreeNode::new(3)));
        let left = Rc::new(RefCell::new(TreeNode::new(9)));
        let right = Rc::new(RefCell::new(TreeNode::new(20)));
        let right_left = Rc::new(RefCell::new(TreeNode::new(15)));
        let right_right = Rc::new(RefCell::new(TreeNode::new(7)));

        right.borrow_mut().left = Some(right_left);
        right.borrow_mut().right = Some(right_right);
        root.borrow_mut().left = Some(left);
        root.borrow_mut().right = Some(right);

        assert_eq!(max_depth(Some(root)), 3);
    }

    #[test]
    fn test_unbalanced_tree() {
        // Create a tree like:
        //     1
        //      \
        //       2
        //        \
        //         3
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let right = Rc::new(RefCell::new(TreeNode::new(2)));
        let right_right = Rc::new(RefCell::new(TreeNode::new(3)));

        right.borrow_mut().right = Some(right_right);
        root.borrow_mut().right = Some(right);

        assert_eq!(max_depth(Some(root)), 3);
    }
}
