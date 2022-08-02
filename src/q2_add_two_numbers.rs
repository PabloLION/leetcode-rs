/*
 * @lc app=leetcode id=2 lang=rust
 *
 * [2] Add Two Numbers
 */

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

struct Solution;
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
pub fn main() {
    assert_eq!(
        Solution::add_two_numbers(
            Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 3, next: None })),
                })),
            })),
            Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 6,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
        ),
        Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 8, next: None })),
            })),
        }))
    )
}
