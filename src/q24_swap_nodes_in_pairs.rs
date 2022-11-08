/*
 * @lc app=leetcode id=24 lang=rust
 *
 * [24] Swap Nodes in Pairs
 */
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    // #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from_vec(vec: Vec<i32>) -> ListNode {
        let mut result = ListNode::new(0);
        let mut tail_ref = &mut result;
        for i in vec {
            let n = Box::new(ListNode::new(i));
            tail_ref.next = Some(n);
            tail_ref = tail_ref.next.as_mut().unwrap();
        }
        result
    }
    pub fn from(vec: Vec<i32>) -> ListNode {
        ListNode::from_vec(vec)
    }
}

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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut count = 0;
        let period = 2;

        let mut dummy = Some(Box::new(ListNode::new(-1)));
        let mut last = &mut dummy;
        let mut next = head;

        while let Some(mut curr_node) = next {
            next = curr_node.next.take();
            curr_node.next = last.as_mut().unwrap().next.take();
            last.as_mut().unwrap().next = Some(curr_node);
            count += 1;
            if count == period {
                while last.as_ref().unwrap().next.is_some() {
                    last = &mut last.as_mut().unwrap().next;
                }
                count = 0;
            }
        }

        dummy.unwrap().next
    }
}
// @lc code=end

struct Solution;
pub fn main() {
    assert_eq!(
        Solution::swap_pairs(Box::new(ListNode::from(vec![1, 2, 3, 4])).next),
        Box::new(ListNode::from(vec![2, 1, 4, 3])).next
    );
    assert_eq!(
        Solution::swap_pairs(Box::new(ListNode::from(vec![])).next),
        Box::new(ListNode::from(vec![])).next
    );
    assert_eq!(
        Solution::swap_pairs(Box::new(ListNode::from(vec![1])).next),
        Box::new(ListNode::from(vec![1])).next
    );
    assert_eq!(
        Solution::swap_pairs(Box::new(ListNode::from(vec![1, 2, 3])).next),
        Box::new(ListNode::from(vec![2, 1, 3])).next
    );
}
