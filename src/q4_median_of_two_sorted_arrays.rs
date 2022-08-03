/*
 * @lc app=leetcode id=4 lang=rust
 *
 * [4] Median of Two Sorted Arrays
 */

// @lc code=start
impl Solution {
    #[allow(dead_code)]
    pub fn find_median_sorted_arrays_copilot(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // @2022-08-03 02:36:00
        // This time the copilot did poorly.
        // This is O(n\ln(n)) time and O(n) space.
        // I think we can at least achieve O(n) time and O(n) space.

        // @2022-08-03 03:49:43
        // The time complexity before was wrong.
        // sort use some method to merge "chunks" (some youtube video said that)

        let mut nums = nums1.clone();
        nums.extend(nums2);
        nums.sort();
        let len = nums.len();
        if len % 2 == 0 {
            (nums[len / 2] + nums[len / 2 - 1]) as f64 / 2.0
        } else {
            nums[len / 2] as f64
        }
    }

    #[allow(dead_code)]
    pub fn find_median_sorted_arrays_pablo(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // inclusive start and exclusive (for 0-size vector slice) end
        let mut start1 = 0;
        let mut end1 = nums1.len();
        let mut start2 = 0;
        let mut end2 = nums2.len();
        let final_state_len = if (nums1.len() + nums2.len()) % 2 == 1 {
            1usize
        } else {
            2usize
        };

        while (end1 - start1) > 0
            && (end2 - start2) > 0
            && (end1 - start1) + (end2 - start2) > final_state_len
        {
            // remove the smallest and the largest number from nums1 and nums2.
            let s1 = nums1[start1];
            let e1 = nums1[end1 - 1];
            let s2 = nums2[start2];
            let e2 = nums2[end2 - 1];

            // prefer to keep the smaller half in nums1 and keep the larger half in nums2
            if s1 <= s2 {
                start1 += 1usize;
            } else {
                start2 += 1usize;
            }
            if e1 > e2 {
                end1 -= 1usize;
            } else {
                end2 -= 1usize;
            }
        }
        let slice1 = &nums1[start1..end1];
        let slice2 = &nums2[start2..end2];
        let mut final_nums: Vec<i32> = slice1.to_vec();
        final_nums.extend(slice2.to_vec());
        let len = final_nums.len();
        if len % 2 == 0 {
            (final_nums[len / 2] + final_nums[len / 2 - 1]) as f64 / 2.0
        } else {
            final_nums[len / 2] as f64
        }
    }

    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        Solution::find_median_sorted_arrays_copilot(nums1, nums2)
    }
}
// @lc code=end

struct Solution;
pub fn main() {
    // assert_eq!(
    //     Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
    //     2.0
    // );
    // assert_eq!(
    //     Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
    //     2.5
    // );
    // assert_eq!(
    //     Solution::find_median_sorted_arrays(vec![0, 0, 0, 0, 0], vec![-1, 0, 0, 0, 0, 0, 1]),
    //     0.0
    // );
    // assert_eq!(
    //     Solution::find_median_sorted_arrays(vec![], vec![1, 2, 3, 4]),
    //     2.5
    // );
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1], vec![2, 3, 4, 5]),
        3.0
    );
}
