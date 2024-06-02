pub struct Solution {}

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        let mut l1 = list1;
        let mut l2 = list2;

        while let (Some(n1), Some(n2)) = (l1.as_ref(), l2.as_ref()) {
            if n1.val <= n2.val {
                tail.next = l1;
                tail = tail.next.as_mut().unwrap();
                l1 = tail.next.take();
            } else {
                tail.next = l2;
                tail = tail.next.as_mut().unwrap();
                l2 = tail.next.take();
            }
        }

        tail.next = if l1.is_some() { l1 } else { l2 };

        dummy.next
    }
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
