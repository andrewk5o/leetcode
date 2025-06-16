// 106. Construct Binary Tree from Inorder and Postorder Traversal
// https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/

use crate::solutions::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn build_tree(
    inorder: Vec<i32>,
    postorder: Vec<i32>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if postorder.is_empty() || inorder.is_empty() {
        return None;
    }

    let root =
        Rc::new(RefCell::new(TreeNode::new(postorder[postorder.len() - 1])));
    let mid_index = inorder
        .iter()
        .position(|&x| x == postorder[postorder.len() - 1])
        .unwrap();

    root.borrow_mut().left = build_tree(
        inorder[..mid_index].to_vec(),
        postorder[..mid_index].to_vec(),
    );
    root.borrow_mut().right = build_tree(
        inorder[mid_index + 1..].to_vec(),
        postorder[mid_index..postorder.len() - 1].to_vec(),
    );

    Some(root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let inorder = vec![9, 3, 15, 20, 7];
        let postorder = vec![9, 15, 7, 20, 3];
        let root = build_tree(inorder, postorder);
        assert_eq!(root.unwrap().borrow().val, 3);
    }

    #[test]
    fn test_example_2() {
        let inorder = vec![-1];
        let postorder = vec![-1];
        let root = build_tree(inorder, postorder);
        assert_eq!(root.unwrap().borrow().val, -1);
    }

    #[test]
    fn test_empty_trees() {
        let inorder = Vec::new();
        let postorder = Vec::new();
        let root = build_tree(inorder, postorder);
        assert!(root.is_none());
    }
}
