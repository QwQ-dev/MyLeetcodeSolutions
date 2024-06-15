use crate::util::linked_list::ListNode;

pub struct Solution {}

impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode::new(-1)));
        let mut tail = &mut dummy_head;
        let mut current = head;
        let mut nodes = Vec::new();

        while let Some(mut node) = current {
            current = node.next.take();
            nodes.push(node);
        }

        nodes.sort_by(|a, b| b.val.cmp(&a.val));

        while let Some(node) = nodes.pop() {
            tail.as_mut().unwrap().next = Some(node);
            tail = &mut tail.as_mut().unwrap().next;
        }

        dummy_head.unwrap().next
    }
}
