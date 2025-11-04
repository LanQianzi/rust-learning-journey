// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
pub fn reverse_between(
    head: Option<Box<ListNode>>,
    left: i32,
    right: i32,
) -> Option<Box<ListNode>> {
    if head.is_none() || head.as_ref().unwrap().next.is_none() {
        return head;
    }
    // 使用虚拟头指针找出暂时列表的头
    let mut dummy = ListNode { val: 0, next: head };
    let mut prev = &mut dummy;
    for _ in 1..left {
        prev = prev.next.as_mut().unwrap();
    }
    let mut temp_head = prev.next.take();

    // 将右端列表单独take出来
    let mut tail = temp_head.as_mut();
    for _ in left..right {
        if let Some(node) = tail {
            tail = node.next.as_mut();
        }
    }
    let mut r_list = tail.unwrap().next.take();

    // 时列表反转
    while let Some(mut node) = temp_head {
        temp_head = node.next;
        node.next = r_list;
        r_list = Some(node);
    }
    prev.next = r_list;
    dummy.next
}
