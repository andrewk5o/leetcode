// 101. Symmetric Tree
// https://leetcode.com/problems/symmetric-tree/

use crate::solutions::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    // Helper function to check if two subtrees are symmetric
    fn is_mirror(
        t1: Option<Rc<RefCell<TreeNode>>>,
        t2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (t1, t2) {
            (None, None) => true,
            (Some(n1), Some(n2)) => {
                let n1 = n1.borrow();
                let n2 = n2.borrow();
                n1.val == n2.val
                    && is_mirror(n1.left.clone(), n2.right.clone())
                    && is_mirror(n1.right.clone(), n2.left.clone())
            }
            _ => false,
        }
    }

    // Handle empty tree and check if left and right subtrees are mirrors
    root.map_or(true, |r| {
        let r = r.borrow();
        is_mirror(r.left.clone(), r.right.clone())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        assert!(is_symmetric(None));
    }

    #[test]
    fn test_single_node() {
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        assert!(is_symmetric(Some(root)));
    }

    #[test]
    fn test_symmetric_tree() {
        // Create a tree like:
        //     1
        //    / \
        //   2   2
        //  / \ / \
        // 3  4 4  3
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let left = Rc::new(RefCell::new(TreeNode::new(2)));
        let right = Rc::new(RefCell::new(TreeNode::new(2)));
        let left_left = Rc::new(RefCell::new(TreeNode::new(3)));
        let left_right = Rc::new(RefCell::new(TreeNode::new(4)));
        let right_left = Rc::new(RefCell::new(TreeNode::new(4)));
        let right_right = Rc::new(RefCell::new(TreeNode::new(3)));

        left.borrow_mut().left = Some(left_left);
        left.borrow_mut().right = Some(left_right);
        right.borrow_mut().left = Some(right_left);
        right.borrow_mut().right = Some(right_right);
        root.borrow_mut().left = Some(left);
        root.borrow_mut().right = Some(right);

        assert!(is_symmetric(Some(root)));
    }

    #[test]
    fn test_asymmetric_tree() {
        // Create a tree like:
        //     1
        //    / \
        //   2   2
        //    \   \
        //     3   3
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let left = Rc::new(RefCell::new(TreeNode::new(2)));
        let right = Rc::new(RefCell::new(TreeNode::new(2)));
        let left_right = Rc::new(RefCell::new(TreeNode::new(3)));
        let right_right = Rc::new(RefCell::new(TreeNode::new(3)));

        left.borrow_mut().right = Some(left_right);
        right.borrow_mut().right = Some(right_right);
        root.borrow_mut().left = Some(left);
        root.borrow_mut().right = Some(right);

        assert!(!is_symmetric(Some(root)));
    }

    #[test]
    fn test_different_values() {
        // Create a tree like:
        //     1
        //    / \
        //   2   3
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let left = Rc::new(RefCell::new(TreeNode::new(2)));
        let right = Rc::new(RefCell::new(TreeNode::new(3)));

        root.borrow_mut().left = Some(left);
        root.borrow_mut().right = Some(right);

        assert!(!is_symmetric(Some(root)));
    }
}
