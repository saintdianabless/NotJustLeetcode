#![allow(dead_code)]

mod oct22;

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

    fn from_vec(data: Vec<i32>) -> Option<Box<Self>> {
        let mut dummy = Self::new(0);
        let mut head = &mut dummy;
        for n in data {
            let next = Some(Box::new(Self::new(n)));
            head.next = next;
            head = head.next.as_mut().unwrap();
        }
        dummy.next
    }
}
struct Solution;
