use std::rc::Rc;
use std::cell::RefCell;

use crate::util::tree_node::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let left_num = Solution::max_depth(node.borrow().left.clone());
                let right_num = Solution::max_depth(node.borrow().right.clone());
                return 1 + std::cmp::max(left_num,right_num);
            }
        }
    }
}