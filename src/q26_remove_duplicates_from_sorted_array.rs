/*
 * @lc app=leetcode id=26 lang=rust
 *
 * [26] Remove Duplicates from Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut len = 1;
        for i in 1..nums.len() {
            if nums[len - 1] != nums[i] {
                nums[len] = nums[i];
                len += 1;
            }
        }

        len as i32
    }
}
// @lc code=end
fn judge_vector(nums: &mut Vec<i32>, expected: Vec<i32>) {
    let len = Solution::remove_duplicates(nums);
    assert_eq!(nums[..len as usize], expected);
}

struct Solution;
pub fn main() {
    judge_vector(&mut vec![1, 1, 2], vec![1, 2]);
    judge_vector(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4], vec![0, 1, 2, 3, 4]);
}
