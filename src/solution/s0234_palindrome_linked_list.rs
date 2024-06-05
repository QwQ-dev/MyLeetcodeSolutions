use crate::util::linked_list::ListNode;

pub struct Solution {}

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut vals = Vec::new();
        let mut curr = &head;
        
        while let Some(node) = curr {
            vals.push(node.val);
            curr = &node.next;
        }
        
        let mut left = 0;
        let mut right = vals.len() - 1;
        
        while left < right {
            if vals[left] != vals[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        
        true
    }
}
