// 226. Invert Binary Tree
// https://leetcode.com/problems/invert-binary-tree/

use crate::solutions::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn invert_tree(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = &root {
        let mut node_ref = node.borrow_mut();

        // Swap left and right subtrees
        let temp = node_ref.left.take();
        node_ref.left = node_ref.right.take();
        node_ref.right = temp;

        // Recursively invert left and right subtrees
        if let Some(left) = &node_ref.left {
            invert_tree(Some(left.clone()));
        }

        if let Some(right) = &node_ref.right {
            invert_tree(Some(right.clone()));
        }
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Helper function to check if two trees are the same
    fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(p), Some(q)) => {
                let p_borrow = p.borrow();
                let q_borrow = q.borrow();
                p_borrow.val == q_borrow.val
                    && is_same_tree(p_borrow.left.clone(), q_borrow.left.clone())
                    && is_same_tree(p_borrow.right.clone(), q_borrow.right.clone())
            }
            _ => false,
        }
    }
    
    // Helper function to create an expected inverted tree for comparison
    fn create_inverted_tree(original: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match original {
            None => None,
            Some(node) => {
                let node_ref = node.borrow();
                let mut inverted = TreeNode::new(node_ref.val);
                
                // Swap left and right and recursively invert them
                inverted.left = create_inverted_tree(node_ref.right.clone());
                inverted.right = create_inverted_tree(node_ref.left.clone());
                
                Some(Rc::new(RefCell::new(inverted)))
            }
        }
    }

    #[test]
    fn test_empty_tree() {
        assert!(is_same_tree(invert_tree(None), None));
    }

    #[test]
    fn test_single_node() {
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let expected = Rc::new(RefCell::new(TreeNode::new(1)));
        
        assert!(is_same_tree(invert_tree(Some(root)), Some(expected)));
    }

    #[test]
    fn test_balanced_tree() {
        // Create a tree like:
        //     4
        //    / \
        //   2   7
        //  / \ / \
        // 1  3 6  9
        let root = Rc::new(RefCell::new(TreeNode::new(4)));
        let left = Rc::new(RefCell::new(TreeNode::new(2)));
        let right = Rc::new(RefCell::new(TreeNode::new(7)));
        let left_left = Rc::new(RefCell::new(TreeNode::new(1)));
        let left_right = Rc::new(RefCell::new(TreeNode::new(3)));
        let right_left = Rc::new(RefCell::new(TreeNode::new(6)));
        let right_right = Rc::new(RefCell::new(TreeNode::new(9)));
        
        left.borrow_mut().left = Some(left_left);
        left.borrow_mut().right = Some(left_right);
        right.borrow_mut().left = Some(right_left);
        right.borrow_mut().right = Some(right_right);
        root.borrow_mut().left = Some(left);
        root.borrow_mut().right = Some(right);
        
        // Create expected inverted tree:
        //     4
        //    / \
        //   7   2
        //  / \ / \
        // 9  6 3  1
        let expected_root = Rc::new(RefCell::new(TreeNode::new(4)));
        let expected_left = Rc::new(RefCell::new(TreeNode::new(7)));
        let expected_right = Rc::new(RefCell::new(TreeNode::new(2)));
        let expected_left_left = Rc::new(RefCell::new(TreeNode::new(9)));
        let expected_left_right = Rc::new(RefCell::new(TreeNode::new(6)));
        let expected_right_left = Rc::new(RefCell::new(TreeNode::new(3)));
        let expected_right_right = Rc::new(RefCell::new(TreeNode::new(1)));
        
        expected_left.borrow_mut().left = Some(expected_left_left);
        expected_left.borrow_mut().right = Some(expected_left_right);
        expected_right.borrow_mut().left = Some(expected_right_left);
        expected_right.borrow_mut().right = Some(expected_right_right);
        expected_root.borrow_mut().left = Some(expected_left);
        expected_root.borrow_mut().right = Some(expected_right);
        
        // Test our invert_tree function
        let inverted = invert_tree(Some(root.clone()));
        assert!(is_same_tree(inverted, Some(expected_root)));
        
        // Also test using our helper function
        // Create a fresh tree for this test to avoid ownership issues
        let fresh_root = Rc::new(RefCell::new(TreeNode::new(4)));
        let fresh_left = Rc::new(RefCell::new(TreeNode::new(2)));
        let fresh_right = Rc::new(RefCell::new(TreeNode::new(7)));
        let fresh_left_left = Rc::new(RefCell::new(TreeNode::new(1)));
        let fresh_left_right = Rc::new(RefCell::new(TreeNode::new(3)));
        let fresh_right_left = Rc::new(RefCell::new(TreeNode::new(6)));
        let fresh_right_right = Rc::new(RefCell::new(TreeNode::new(9)));
        
        fresh_left.borrow_mut().left = Some(fresh_left_left);
        fresh_left.borrow_mut().right = Some(fresh_left_right);
        fresh_right.borrow_mut().left = Some(fresh_right_left);
        fresh_right.borrow_mut().right = Some(fresh_right_right);
        fresh_root.borrow_mut().left = Some(fresh_left);
        fresh_root.borrow_mut().right = Some(fresh_right);
        
        let expected_with_helper = create_inverted_tree(Some(fresh_root.clone()));
        let fresh_inverted = invert_tree(Some(fresh_root));
        assert!(is_same_tree(fresh_inverted, expected_with_helper));
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
        
        // Expected inverted tree:
        //     1
        //    /
        //   2
        //  /
        // 3
        let expected_root = Rc::new(RefCell::new(TreeNode::new(1)));
        let expected_left = Rc::new(RefCell::new(TreeNode::new(2)));
        let expected_left_left = Rc::new(RefCell::new(TreeNode::new(3)));
        
        expected_left.borrow_mut().left = Some(expected_left_left);
        expected_root.borrow_mut().left = Some(expected_left);
        
        assert!(is_same_tree(invert_tree(Some(root)), Some(expected_root)));
    }
    
    #[test]
    fn test_double_inversion() {
        // Test that inverting a tree twice returns the original tree
        let root = Rc::new(RefCell::new(TreeNode::new(4)));
        let left = Rc::new(RefCell::new(TreeNode::new(2)));
        let right = Rc::new(RefCell::new(TreeNode::new(7)));
        
        root.borrow_mut().left = Some(left);
        root.borrow_mut().right = Some(right);
        
        // Clone the original tree for comparison
        let original_clone = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let original_left = Rc::new(RefCell::new(TreeNode::new(2)));
        let original_right = Rc::new(RefCell::new(TreeNode::new(7)));
        
        original_clone.as_ref().unwrap().borrow_mut().left = Some(original_left);
        original_clone.as_ref().unwrap().borrow_mut().right = Some(original_right);
        
        // Invert twice
        let twice_inverted = invert_tree(invert_tree(Some(root)));
        
        // Should be the same as the original
        assert!(is_same_tree(twice_inverted, original_clone));
    }
}
