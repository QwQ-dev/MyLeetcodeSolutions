use crate::util::linked_list::ListNode;

pub struct Solution {}

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut prev = &mut dummy;
        let mut current = prev.next.take();

        while let Some(mut first) = current {
            if let Some(mut second) = first.next.take() {
                first.next = second.next.take();
                second.next = Some(first);
                prev.next = Some(second);

                prev = prev.next.as_mut().unwrap().next.as_mut().unwrap();
                current = prev.next.take();
            } else {
                prev.next = Some(first);
                break;
            }
        }

        dummy.next
    }
}