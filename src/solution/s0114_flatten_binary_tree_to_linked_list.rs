use std::rc::Rc;
use std::cell::RefCell;

use crate::util::tree_node::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {
            return;
        }

        let mut prev: Option<Rc<RefCell<TreeNode>>> = None;

        fn flatten_helper(node: Option<Rc<RefCell<TreeNode>>>, prev: &mut Option<Rc<RefCell<TreeNode>>>) {
            if let Some(curr) = node {
                let right = curr.borrow_mut().right.take();
                let left = curr.borrow_mut().left.take();

                if let Some(p) = prev.take() {
                    p.borrow_mut().right = Some(curr.clone());
                }
                *prev = Some(curr);

                flatten_helper(left, prev);
                flatten_helper(right, prev);
            }
        }

        flatten_helper(root.clone(), &mut prev);
    }
}