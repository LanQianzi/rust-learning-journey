// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut list1 = head;
    let mut list2 = None;
    while let Some(mut node1) = list1 {
        list1 = node1.next;
        node1.next = list2;
        list2 = Some(node1);
    }
    list2
}
