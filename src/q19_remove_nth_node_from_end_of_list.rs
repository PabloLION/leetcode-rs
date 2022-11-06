/*
 * @lc app=leetcode id=19 lang=rust
 *
 * [19] Remove Nth Node From End of List
 */

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    // #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn from_vec(vec: Vec<i32>) -> ListNode {
        let mut result = ListNode::new(0);
        let mut tail_ref = &mut result;
        for i in vec {
            let n = Box::new(ListNode::new(i));
            tail_ref.next = Some(n);
            tail_ref = tail_ref.next.as_mut().unwrap();
        }
        result
    }
    fn from(vec: Vec<i32>) -> ListNode {
        ListNode::from_vec(vec)
    }
}

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }

// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut tail = &mut head;
        let mut count: usize = 0;
        while tail.is_some() {
            count += 1;
            tail = &mut tail.as_mut().unwrap().next;
        }
        if count == n as usize {
            return head.unwrap().next;
        }
        let mut tail = &mut head;
        for _ in 0..count - (n as usize + 1) {
            tail = &mut tail.as_mut().unwrap().next;
        }
        let mut node = tail.as_mut().unwrap().next.take();
        tail.as_mut().unwrap().next = node.as_mut().unwrap().next.take();
        head
    }
}
// @lc code=end

struct Solution;
pub fn main() {
    assert_eq!(
        Solution::remove_nth_from_end(Box::new(ListNode::from(vec![1, 2, 3, 4, 5])).next, 2),
        Box::new(ListNode::from(vec![1, 2, 3, 5])).next
    );
    assert_eq!(
        Solution::remove_nth_from_end(Box::new(ListNode::from(vec![1])).next, 1),
        Box::new(ListNode::from(vec![])).next
    );
    assert_eq!(
        Solution::remove_nth_from_end(Box::new(ListNode::from(vec![1, 2])).next, 2),
        Box::new(ListNode::from(vec![2])).next
    );
}
