// 105. Construct Binary Tree from Preorder and Inorder Traversal
// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/

use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn build_tree(
    preorder: Vec<i32>,
    inorder: Vec<i32>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if preorder.is_empty() || inorder.is_empty() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(preorder[0])));
    let mid_index = inorder.iter().position(|&x| x == preorder[0]).unwrap();

    root.borrow_mut().left = build_tree(
        preorder[1..=mid_index].to_vec(),
        inorder[..mid_index].to_vec(),
    );
    root.borrow_mut().right = build_tree(
        preorder[mid_index + 1..].to_vec(),
        inorder[mid_index + 1..].to_vec(),
    );

    Some(root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];
        let root = build_tree(preorder, inorder);
        assert_eq!(root.unwrap().borrow().val, 3);
    }

    #[test]
    fn test_example_2() {
        let preorder = vec![-1];
        let inorder = vec![-1];
        let root = build_tree(preorder, inorder);
        assert_eq!(root.unwrap().borrow().val, -1);
    }

    #[test]
    fn test_empty_trees() {
        let preorder = Vec::new();
        let inorder = Vec::new();
        let root = build_tree(preorder, inorder);
        assert!(root.is_none());
    }
}
