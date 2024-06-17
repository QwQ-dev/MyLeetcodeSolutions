use std::rc::Rc;
use std::cell::RefCell;

use crate::util::tree_node::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(node) => {
                let left = node.borrow().left.clone();
                let right = node.borrow().right.clone();
                
                node.borrow_mut().left = Solution::invert_tree(right);
                node.borrow_mut().right = Solution::invert_tree(left);
                
                Some(node)
            },
            None => None,
        }
    }
}