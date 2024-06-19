use std::rc::Rc;
use std::cell::RefCell;

use crate::util::tree_node::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if nums.is_empty() {
                return None;
            }

            let mid = nums.len() / 2;
            let root = Rc::new(RefCell::new(TreeNode::new(nums[mid])));
            root.borrow_mut().left = helper(&nums[..mid]);
            root.borrow_mut().right = helper(&nums[mid+1..]);

            Some(root)
        }

        helper(&nums)
    }
}