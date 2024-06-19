use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

use crate::util::tree_node::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        if root.is_none() {
            return result;
        }

        let mut queue = VecDeque::new();
        queue.push_back(root);

        while !queue.is_empty() {
            let level_size = queue.len();
            let mut current_level = Vec::new();

            for _ in 0..level_size {
                if let Some(Some(node)) = queue.pop_front() {
                    let node = node.borrow();
                    current_level.push(node.val);
                    if node.left.is_some() {
                        queue.push_back(node.left.clone());
                    }
                    if node.right.is_some() {
                        queue.push_back(node.right.clone());
                    }
                }
            }
            result.push(current_level);
        }

        result
    }
}