/*
 * @lc app=leetcode id=25 lang=rust
 *
 * [25] Reverse Nodes in k-Group
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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut count = 0;
        let mut len = 0;
        let period = k;

        let mut next = head.as_ref();

        while let Some(curr_node) = next {
            next = curr_node.next.as_ref();
            len += 1;
        }
        let stop = (len / period) * period;

        let mut dummy = Some(Box::new(ListNode::new(-1)));
        let mut period_end = &mut dummy;
        let mut next = head;
        while let Some(mut curr_node) = next {
            next = curr_node.next.take();
            curr_node.next = period_end.as_mut().unwrap().next.take(); // curr_node of last iter
            period_end.as_mut().unwrap().next = Some(curr_node);
            count += 1;
            println!("count: {}, stop: {}, period: {}", count, stop, period);
            if count % period == 0 {
                while period_end.as_ref().unwrap().next.is_some() {
                    period_end = &mut period_end.as_mut().unwrap().next;
                }
            }
            if count == stop {
                period_end.as_mut().unwrap().next = next;
                break;
            }
        }

        // undo the last "count" nodes
        // 5432[1]9876
        // while count > 1 {
        //     let mut tail = period_end.clone(); // update in iter
        //     let mut penultimate_node: Box<ListNode> = loop {
        //         // let next = node.next;
        //         if tail.as_ref().unwrap().next.as_ref().unwrap().next.is_some() {
        //             tail = Some(tail.unwrap().next.unwrap());
        //         } else {
        //             break tail.unwrap();
        //         }
        //     }; // tail_node -> None
        //     penultimate_node.next.unwrap().next = period_end.as_mut().unwrap().next.take();
        //     println!("");
        //     // println!("tail_node: {:?}", penultimate_node);
        //     period_end.as_mut().unwrap().next = Some(penultimate_node);
        //     println!("period_end: {:?}", period_end);
        //     period_end = &mut period_end.as_mut().unwrap().next;
        //     count -= 1;
        // }
        dummy.unwrap().next
    }
}
// @lc code=end

struct Solution;
pub fn main() {
    assert_eq!(
        Solution::reverse_k_group(
            Box::new(ListNode::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9])).next,
            5
        ),
        Box::new(ListNode::from(vec![5, 4, 3, 2, 1, 6, 7, 8, 9])).next
    );
    assert_eq!(
        Solution::reverse_k_group(Box::new(ListNode::from(vec![1, 2, 3, 4, 5, 6, 7])).next, 4),
        Box::new(ListNode::from(vec![4, 3, 2, 1, 5, 6, 7])).next
    );
    assert_eq!(
        Solution::reverse_k_group(Box::new(ListNode::from(vec![1, 2, 3, 4, 5])).next, 2),
        Box::new(ListNode::from(vec![2, 1, 4, 3, 5])).next
    );
    assert_eq!(
        Solution::reverse_k_group(Box::new(ListNode::from(vec![1, 2, 3, 4, 5])).next, 3),
        Box::new(ListNode::from(vec![3, 2, 1, 4, 5])).next
    );
}
