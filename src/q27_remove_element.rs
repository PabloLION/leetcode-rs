/*
 * @lc app=leetcode id=27 lang=rust
 *
 * [27] Remove Element
 */

// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|&x| x != val);
        nums.len() as i32
    }
}
// @lc code=end

pub fn judge_vector(nums: &mut Vec<i32>, to_remove: i32, expected: Vec<i32>) {
    // let len =
    Solution::remove_element(nums, to_remove);
    // let mut new_nums = nums.to_vec();
    nums.sort();
    let expected = {
        let mut expected = expected;
        expected.sort();
        expected
    };
    assert_eq!(nums.clone(), expected);
}

struct Solution;
pub fn main() {
    judge_vector(&mut vec![3, 2, 2, 3], 3, vec![2, 2]);
    judge_vector(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2, vec![0, 1, 3, 0, 4]);
}
