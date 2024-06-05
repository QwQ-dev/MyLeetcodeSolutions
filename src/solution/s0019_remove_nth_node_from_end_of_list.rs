use crate::util::linked_list::ListNode;

pub struct Solution {}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut fast = dummy.clone();
        let mut slow = dummy.as_mut();

        for _ in 0..n {
            if let Some(ref next) = fast.next {
                fast = next.clone();
            }
        }

        while fast.next.is_some() {
            fast = fast.next.unwrap();
            slow = slow.next.as_mut().unwrap();
        }

        let next = slow.next.as_mut().unwrap().next.take();
        slow.next = next;

        dummy.next
    }
}
