#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

pub fn merge_two_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (None, None) => None,
        (Some(n), None) | (None, Some(n)) => Some(n),
        (Some(l1), Some(l2)) => {
            if l1.val >= l2.val {
                Some(Box::new(ListNode {
                    val: l2.val,
                    next: merge_two_lists(Some(l1), l2.next),
                }))
            } else {
                Some(Box::new(ListNode {
                    val: l1.val,
                    next: merge_two_lists(l1.next, Some(l2)),
                }))
            }
        }
    }
}
