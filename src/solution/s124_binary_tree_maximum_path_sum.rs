use std::rc::Rc;
use std::cell::RefCell;
use std::cmp;

use crate::util::tree_node::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_sum = std::i32::MIN;
        Self::max_path_sum_recursive(&root, &mut max_sum);
        max_sum
    }
    
    fn max_path_sum_recursive(node: &Option<Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> i32 {
        if let Some(n) = node {
            let n = n.borrow();
            let left_sum = cmp::max(0, Self::max_path_sum_recursive(&n.left, max_sum));
            let right_sum = cmp::max(0, Self::max_path_sum_recursive(&n.right, max_sum));
            
            *max_sum = cmp::max(*max_sum, n.val + left_sum + right_sum);
            
            n.val + cmp::max(left_sum, right_sum)
        } else {
            0
        }
    }
}