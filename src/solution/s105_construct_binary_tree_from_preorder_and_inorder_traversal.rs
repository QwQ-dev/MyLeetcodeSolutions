use std::rc::Rc;
use std::cell::RefCell;

use crate::util::tree_node::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree_helper(&preorder, &inorder, 0, 0, inorder.len() as i32)
    }
    
    fn build_tree_helper(preorder: &[i32], inorder: &[i32], prestart: i32, instart: i32, inend: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if prestart >= preorder.len() as i32 || instart >= inend {
            return None;
        }
        
        let root_val = preorder[prestart as usize];
        let mut root = TreeNode::new(root_val);
        
        let mut root_index = instart;
        while root_index < inend && inorder[root_index as usize] != root_val {
            root_index += 1;
        }
        
        root.left = Self::build_tree_helper(preorder, inorder, prestart + 1, instart, root_index);
        root.right = Self::build_tree_helper(preorder, inorder, prestart + (root_index - instart) + 1, root_index + 1, inend);
        
        Some(Rc::new(RefCell::new(root)))
    }
}