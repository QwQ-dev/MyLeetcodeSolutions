use crate::util::linked_list::ListNode;

pub struct Solution {}

impl Solution {
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        fn reverse(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
            let mut prev = None;
            let mut curr = head;
            for _ in 0..k {
                if curr.is_none() {
                    return prev;
                }
                let next = curr.as_mut().unwrap().next.take();
                curr.as_mut().unwrap().next = prev;
                prev = curr;
                curr = next;
            }
            prev
        }

        let mut node = &head;
        let mut length = 0;
        while let Some(ref n) = node {
            length += 1;
            node = &n.next;
        }

        if k <= 1 || length < k {
            return head;
        }

        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut group_prev = &mut dummy;
        while length >= k {
            let group_head = group_prev.as_mut().unwrap().next.take();
            let mut group_tail = group_head.clone();
            for _ in 1..k {
                group_tail = group_tail.unwrap().next;
            }
            let next_group = group_tail.unwrap().next.take();
            group_prev.as_mut().unwrap().next = reverse(group_head, k);
            while group_prev.as_mut().unwrap().next.is_some() {
                group_prev = &mut group_prev.as_mut().unwrap().next;
            }
            group_prev.as_mut().unwrap().next = next_group;
            length -= k;
        }

        dummy.unwrap().next
    }
}