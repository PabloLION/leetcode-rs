/*
 * @lc app=leetcode id=2 lang=rust
 *
 * [2] Add Two Numbers
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

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = Box::new(ListNode::new(0));
        let mut tail_ref = &mut result;

        let mut l1 = l1;
        let mut l2 = l2;
        let mut carry = 0;

        while l1.is_some() && l2.is_some() {
            let ll1 = l1.unwrap();
            let ll2 = l2.unwrap();

            let sum = ll1.val + ll2.val + carry;
            carry = sum / 10;
            let n = Box::new(ListNode::new(sum % 10));
            tail_ref.next = Some(n);
            tail_ref = tail_ref.next.as_mut().unwrap();

            l1 = ll1.next;
            l2 = ll2.next;
        }

        let mut l = if l1.is_some() { l1 } else { l2 };
        while l.is_some() {
            let ll = l.unwrap();
            let sum = ll.val + carry;
            carry = sum / 10;
            let n = Box::new(ListNode::new(sum % 10));
            tail_ref.next = Some(n);
            tail_ref = tail_ref.next.as_mut().unwrap();
            l = ll.next;
        }

        if carry != 0 {
            tail_ref.next = Some(Box::new(ListNode::new(carry)));
        }

        result.next
    }
}
// @lc code=end

struct Solution;
pub fn main() {
    assert_eq!(
        Solution::add_two_numbers(
            Box::new(ListNode::from(vec![2, 4, 3])).next,
            Box::new(ListNode::from(vec![5, 6, 4])).next,
        ),
        Box::new(ListNode::from(vec![7, 0, 8])).next
    )
}
