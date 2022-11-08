/*
 * @lc app=leetcode id=23 lang=rust
 *
 * [23] Merge k Sorted Lists
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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut result = Box::new(ListNode::new(0));
        let mut tail_ref = &mut result;

        let mut last_nodes = lists;
        while last_nodes.iter().any(|x| x.is_some()) {
            let mut min = std::i32::MAX;
            let mut min_index = 0;
            for (i, node) in last_nodes.iter().enumerate() {
                if let Some(n) = node {
                    if n.val < min {
                        min = n.val;
                        min_index = i;
                    }
                }
            }

            if let Some(n) = last_nodes[min_index].take() {
                tail_ref.next = Some(n);
                tail_ref = tail_ref.next.as_mut().unwrap();
                last_nodes[min_index] = tail_ref.next.take();
            }
        }

        result.next
    }
}
// @lc code=end

struct Solution;
pub fn main() {
    assert_eq!(
        Solution::merge_k_lists(vec![
            Box::new(ListNode::from(vec![1, 4, 5])).next,
            Box::new(ListNode::from(vec![1, 3, 4])).next,
            Box::new(ListNode::from(vec![2, 6])).next
        ]),
        Box::new(ListNode::from(vec![1, 1, 2, 3, 4, 4, 5, 6])).next
    );
    assert_eq!(
        Solution::merge_k_lists(vec![Box::new(ListNode::from(vec![])).next,]),
        Box::new(ListNode::from(vec![])).next
    );
    assert_eq!(
        Solution::merge_k_lists(vec![]),
        Box::new(ListNode::from(vec![])).next
    );
}
