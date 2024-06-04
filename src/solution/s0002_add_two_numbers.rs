pub struct Solution {}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        let mut carry = 0;

        let mut p1 = l1;
        let mut p2 = l2;

        while p1.is_some() || p2.is_some() || carry != 0 {
            let sum = carry 
                + p1.as_ref().map_or(0, |node| node.val) 
                + p2.as_ref().map_or(0, |node| node.val);

            carry = sum / 10;
            let new_node = Box::new(ListNode::new(sum % 10));
            tail.next = Some(new_node);
            tail = tail.next.as_mut().unwrap();

            p1 = p1.and_then(|node| node.next);
            p2 = p2.and_then(|node| node.next);
        }

        dummy.next
    }
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}