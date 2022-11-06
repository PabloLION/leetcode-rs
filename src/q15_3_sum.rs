/*
 * @lc app=leetcode id=15 lang=rust
 *
 * [15] 3Sum
 */

// @lc code=start
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        if nums.len() < 3 {
            return ans;
        }
        let mut nums = nums;
        nums.sort();
        let mut i = 0;
        while i < nums.len() - 2 {
            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                if sum == 0 {
                    ans.push(vec![nums[i], nums[j], nums[k]]);
                    while j < k && nums[j] == nums[j + 1] {
                        j += 1;
                    }
                    while j < k && nums[k] == nums[k - 1] {
                        k -= 1;
                    }
                    j += 1;
                    k -= 1;
                } else if sum < 0 {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
            while i < nums.len() - 2 && nums[i] == nums[i + 1] {
                i += 1;
            }
            i += 1;
        }
        ans
    }
}
// @lc code=end
struct Solution;
pub fn main() {
    assert_eq!(Solution::three_sum(vec![0, 1, 1]), Vec::<Vec<i32>>::new());
    assert_eq!(Solution::three_sum(vec![0, 0, 0, 0]), vec![vec![0, 0, 0]]);
    assert_eq!(
        Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
        vec![vec![-1, -1, 2], vec![-1, 0, 1]]
    );
}
