use std::rc::Rc;
use std::cell::RefCell;

use crate::util::tree_node::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn height_and_diameter(node: Option<Rc<RefCell<TreeNode>>>, diameter: &mut i32) -> i32 {
            if let Some(n) = node {
                let n = n.borrow();
                let left_height = height_and_diameter(n.left.clone(), diameter);
                let right_height = height_and_diameter(n.right.clone(), diameter);
                *diameter = (*diameter).max(left_height + right_height);
                return 1 + left_height.max(right_height);
            }
            0
        }

        let mut diameter = 0;
        height_and_diameter(root, &mut diameter);
        diameter
    }
}