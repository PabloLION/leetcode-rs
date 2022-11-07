/*
 * @lc app=leetcode id=21 lang=rust
 *
 * [21] Merge Two Sorted Lists
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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = Box::new(ListNode::new(0));
        let mut tail_ref = &mut result;

        let mut l1 = list1;
        let mut l2 = list2;

        while l1.is_some() && l2.is_some() {
            let n1 = l1.clone().unwrap();
            let n2 = l2.clone().unwrap();

            tail_ref.next = {
                if n1.val < n2.val {
                    let c = l1.clone();
                    l1 = n1.next;
                    c
                } else {
                    let c = l2.clone();
                    l2 = n2.next;
                    c
                }
            };
            tail_ref = tail_ref.next.as_mut().unwrap();
        }
        // tail_ref = tail_ref.next.as_mut().unwrap();

        let l = if l1.is_some() { l1 } else { l2 };
        tail_ref.next = l;

        result.next
    }
}
// @lc code=end

struct Solution;
pub fn main() {
    assert_eq!(
        Solution::merge_two_lists(
            Box::new(ListNode::from(vec![1, 2, 4])).next,
            Box::new(ListNode::from(vec![1, 3, 4])).next,
        ),
        Box::new(ListNode::from(vec![1, 1, 2, 3, 4, 4])).next
    );
    assert_eq!(
        Solution::merge_two_lists(
            Box::new(ListNode::from(vec![])).next,
            Box::new(ListNode::from(vec![])).next,
        ),
        Box::new(ListNode::from(vec![])).next
    );
    assert_eq!(
        Solution::merge_two_lists(
            Box::new(ListNode::from(vec![])).next,
            Box::new(ListNode::from(vec![0])).next,
        ),
        Box::new(ListNode::from(vec![0])).next
    );
}
