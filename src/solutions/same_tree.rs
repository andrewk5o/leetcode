// 100. Same Tree
// https://leetcode.com/problems/same-tree/

use std::cell::RefCell;
use std::rc::Rc;

use crate::solutions::tree_node::TreeNode;

pub fn is_same_tree(
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    match (p, q) {
        (None, None) => true,
        (Some(p), Some(q)) => {
            let p = p.borrow();
            let q = q.borrow();
            p.val == q.val
                && is_same_tree(p.left.clone(), q.left.clone())
                && is_same_tree(p.right.clone(), q.right.clone())
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_trees() {
        assert!(is_same_tree(None, None));
    }

    #[test]
    fn test_single_node_same() {
        let p = Rc::new(RefCell::new(TreeNode::new(1)));
        let q = Rc::new(RefCell::new(TreeNode::new(1)));
        assert!(is_same_tree(Some(p), Some(q)));
    }

    #[test]
    fn test_single_node_different() {
        let p = Rc::new(RefCell::new(TreeNode::new(1)));
        let q = Rc::new(RefCell::new(TreeNode::new(2)));
        assert!(!is_same_tree(Some(p), Some(q)));
    }

    #[test]
    fn test_complex_trees() {
        // Create first tree:
        //     1
        //    / \
        //   2   3
        let p = Rc::new(RefCell::new(TreeNode::new(1)));
        let p_left = Rc::new(RefCell::new(TreeNode::new(2)));
        let p_right = Rc::new(RefCell::new(TreeNode::new(3)));
        p.borrow_mut().left = Some(p_left);
        p.borrow_mut().right = Some(p_right);

        // Create second tree:
        //     1
        //    / \
        //   2   3
        let q = Rc::new(RefCell::new(TreeNode::new(1)));
        let q_left = Rc::new(RefCell::new(TreeNode::new(2)));
        let q_right = Rc::new(RefCell::new(TreeNode::new(3)));
        q.borrow_mut().left = Some(q_left);
        q.borrow_mut().right = Some(q_right);

        assert!(is_same_tree(Some(p), Some(q)));
    }

    #[test]
    fn test_different_structure() {
        // Create first tree:
        //     1
        //    /
        //   2
        let p = Rc::new(RefCell::new(TreeNode::new(1)));
        let p_left = Rc::new(RefCell::new(TreeNode::new(2)));
        p.borrow_mut().left = Some(p_left);

        // Create second tree:
        //     1
        //      \
        //       2
        let q = Rc::new(RefCell::new(TreeNode::new(1)));
        let q_right = Rc::new(RefCell::new(TreeNode::new(2)));
        q.borrow_mut().right = Some(q_right);

        assert!(!is_same_tree(Some(p), Some(q)));
    }
}
