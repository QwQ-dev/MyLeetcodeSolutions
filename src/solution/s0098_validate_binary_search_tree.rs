use std::rc::Rc;
use std::cell::RefCell;

use crate::util::tree_node::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn is_valid_bst_helper(node: Option<Rc<RefCell<TreeNode>>>, min: Option<i32>, max: Option<i32>) -> bool {
            match node {
                Some(n) => {
                    let n = n.borrow();
                    let val = n.val;
                    
                    if let Some(min_val) = min {
                        if val <= min_val {
                            return false;
                        }
                    }
                    
                    if let Some(max_val) = max {
                        if val >= max_val {
                            return false;
                        }
                    }
                    
                    if !is_valid_bst_helper(n.left.clone(), min, Some(val)) {
                        return false;
                    }
                    
                    if !is_valid_bst_helper(n.right.clone(), Some(val), max) {
                        return false;
                    }
                    
                    true
                }
                None => true,
            }
        }
        
        is_valid_bst_helper(root, None, None)
    }
}