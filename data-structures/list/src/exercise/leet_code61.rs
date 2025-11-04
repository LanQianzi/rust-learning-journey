// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    // 获取长度
    let mut len = 0;
    let mut p = &head;
    while p.is_some() {
        p = &p.as_ref().unwrap().next;
        len += 1;
    }
    if len == 0 {
        return None;
    }
    let rot = k % len;
    if rot == 0 {
        return head;
    }

    // 得到新的头节点
    let mut head = head;
    let mut slow = &mut head;
    for _ in 0..(len - rot) {
        slow = &mut slow.as_mut().unwrap().next
    }
    let mut new_head = slow.take();

    // 获取尾节点，将原来的头节点接到尾节点的后面
    let mut tail = &mut new_head;
    while tail.as_ref().unwrap().next.is_some() {
        tail = &mut tail.as_mut().unwrap().next
    }
    tail.as_mut().unwrap().next = head;

    new_head
}
