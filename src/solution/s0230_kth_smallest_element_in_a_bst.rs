use std::rc::Rc;
use std::cell::RefCell;

use crate::util::tree_node::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        fn inorder_traversal(node: Option<Rc<RefCell<TreeNode>>>, k: &mut i32, result: &mut Option<i32>) {
            if let Some(n) = node {
                let n = n.borrow();
                
                inorder_traversal(n.left.clone(), k, result);
                
                *k -= 1;
                if *k == 0 {
                    *result = Some(n.val);
                    return;
                }
                
                inorder_traversal(n.right.clone(), k, result);
            }
        }
        
        let mut k = k;
        let mut result = None;
        inorder_traversal(root, &mut k, &mut result);
        result.unwrap()
    }
}