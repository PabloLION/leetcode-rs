/*
 * @lc app=leetcode id=2 lang=rust
 *
 * [2] Add Two Numbers
 */

use crate::ListNode;
use crate::Solution;

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

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

        while l1.is_some() {
            let ll1 = l1.unwrap();
            let sum = ll1.val + carry;
            carry = sum / 10;
            let n = Box::new(ListNode::new(sum % 10));
            tail_ref.next = Some(n);
            tail_ref = tail_ref.next.as_mut().unwrap();
            l1 = ll1.next;
        }

        while l2.is_some() {
            let ll2 = l2.unwrap();
            let sum = ll2.val + carry;
            carry = sum / 10;
            let n = Box::new(ListNode::new(sum % 10));
            tail_ref.next = Some(n);
            tail_ref = tail_ref.next.as_mut().unwrap();
            l2 = ll2.next;
        }

        if carry != 0 {
            tail_ref.next = Some(Box::new(ListNode::new(carry)));
        }

        result.next
    }
}
// @lc code=end
