use std::rc::Rc;
use std::cell::RefCell;

use crate::util::tree_node::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }
        
        Self::is_mirror(&root.as_ref().unwrap().borrow().left, &root.as_ref().unwrap().borrow().right)
    }

    fn is_mirror(left: &Option<Rc<RefCell<TreeNode>>>, right: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (left, right) {
            (None, None) => true,
            (Some(l), Some(r)) => {
                l.borrow().val == r.borrow().val &&
                Self::is_mirror(&l.borrow().left, &r.borrow().right) &&
                Self::is_mirror(&l.borrow().right, &r.borrow().left)
            },
            _ => false,
        }
    }
}