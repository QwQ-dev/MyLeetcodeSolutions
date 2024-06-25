use std::rc::Rc;
use std::cell::RefCell;

use crate::util::tree_node::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, target: i32, current_sum: i64, sum_map: &mut std::collections::HashMap<i64, i32>) -> i32 {
            if let Some(n) = node {
                let n = n.borrow();
                let current_sum = current_sum + n.val as i64;
                
                let mut count = *sum_map.get(&(current_sum - target as i64)).unwrap_or(&0);
                
                if current_sum == target as i64 {
                    count += 1;
                }
                
                *sum_map.entry(current_sum).or_insert(0) += 1;
                
                count += dfs(&n.left, target, current_sum, sum_map);
                count += dfs(&n.right, target, current_sum, sum_map);
                
                *sum_map.get_mut(&current_sum).unwrap() -= 1;
                
                count
            } else {
                0
            }
        }
        
        let mut sum_map = std::collections::HashMap::new();
        dfs(&root, target_sum, 0, &mut sum_map)
    }
}