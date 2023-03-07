struct Solution;

// Definition for singly-linked list.
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
}

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let ret_list = Box::new(ListNode::new(5));

        Some(ret_list)
    }
}
fn main() {
    let list1: Option<Box<ListNode>> = Some(Box::new(ListNode::new(5)));
    let list2: Option<Box<ListNode>> = Some(Box::new(ListNode::new(5)));
    let temp = Solution::merge_two_lists(list1, list2);
    println!("{:#?}", temp)
}
