// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let dummy = ListNode { val: 0, next: head };
    let mut left = &dummy;
    let mut right = &dummy;
    for _ in 0..n {
        right = right.next.as_ref().unwrap();
    }
    while right.next.is_some() {
        left = left.next.as_ref().unwrap();
        right = right.next.as_ref().unwrap();
    }

    #[allow(mutable_transmutes)]
    let left: &mut ListNode = unsafe { std::mem::transmute(left) };
    left.next = left.next.take()?.next;
    dummy.next
}
