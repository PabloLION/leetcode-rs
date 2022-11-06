/*
 * @lc app=leetcode id=18 lang=rust
 *
 * [18] 4Sum
 */

// @lc code=start
impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::<Vec<i32>>::new();
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        for i in 0..n {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            for j in i + 1..n {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }
                let mut k = j + 1;
                let mut l = n - 1;
                while k < l {
                    let mut sum: i32 = 0;
                    let mut overflowed = false;
                    for idx in [i, j, k, l].iter() {
                        let result = sum.overflowing_add(nums[*idx]);
                        sum = result.0;
                        overflowed = overflowed || result.1;
                    }
                    if overflowed {
                        break;
                    }
                    if sum == target {
                        ans.push(vec![nums[i], nums[j], nums[k], nums[l]]);
                        k += 1;
                        while k < l && nums[k] == nums[k - 1] {
                            k += 1;
                        }
                        l -= 1;
                        while k < l && nums[l] == nums[l + 1] {
                            l -= 1;
                        }
                    } else if sum < target {
                        k += 1;
                    } else {
                        l -= 1;
                    }
                }
            }
        }
        ans
    }
}
// @lc code=end

struct Solution;
pub fn main() {
    assert_eq!(
        Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0).sort(),
        vec![vec![-1, 0, 0, 1], vec![-2, -1, 1, 2], vec![-2, 0, 0, 2]].sort()
    );
    assert_eq!(
        Solution::four_sum(vec![0, 0, 0, 0], 0),
        vec![vec![0, 0, 0, 0]]
    );
    assert_eq!(
        Solution::four_sum(
            vec![1000000000, 1000000000, 1000000000, 1000000000],
            -294967296
        ),
        Vec::<Vec<i32>>::new()
    );
}
