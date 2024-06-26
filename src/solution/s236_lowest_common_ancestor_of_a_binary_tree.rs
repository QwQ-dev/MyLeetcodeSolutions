use std::rc::Rc;
use std::cell::RefCell;

use crate::util::tree_node::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() || root == p || root == q {
            return root;
        }

        let root_ref = root.as_ref().unwrap();
        let left = Self::lowest_common_ancestor(
            root_ref.borrow().left.clone(),
            p.clone(),
            q.clone()
        );
        
        let right = Self::lowest_common_ancestor(
            root_ref.borrow().right.clone(),
            p.clone(),
            q.clone()
        );

        if left.is_some() && right.is_some() {
            return root;
        }

        if left.is_some() {
            left
        } else {
            right
        }
    }
}