/*
 * @lc app=leetcode id=4 lang=rust
 *
 * [4] Median of Two Sorted Arrays
 */

/* observation1:
 * Recursion:
 * Removing the smallest and the largest element from the two sorted arrays will
 * give us a new problem of the same type but with two less elements.
 * The termination condition is when the sum of the two arrays is 1 or 2.
 *
 * Complexity: O(n) for time and O(1) for space
 */

/* observation2:
 * `l` is the sum of length of two arrays
 * `h` is the number of the elements contained in the "smaller" half.
 * h=$(l-1)/2$ for odd $l$ and $(l-2)/2$ for even $l$
 * If we can find the smaller half `h` elements and remove it, in the remaining array,
 * the mean of the smallest one(odd $l$) or two(even $l$) numbers is the median.
 *
 * We can search for an x, such that (natural index)
 * nums1[x] <= nums2[y+1] and nums2[y] <= nums1[x+1], where y = h-x
 *
 * These x numbers in nums1 and y numbers in nums2 form the smallest half.
 * The median is the mean of smallest two numbers remaining.
 * Apply binary search for x make it faster.
 *
 * Complexity: O(\ln(m+n)) for time and O(1) for space
 */

// @lc code=start

use std::usize;

impl Solution {
    #[allow(dead_code)]
    pub fn find_median_sorted_arrays_copilot(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // @2022-08-03 02:36:00
        // This time the copilot did poorly.
        // This is O(n\ln(n)) time and O(n) space.
        // I think we can at least achieve O(n) time and O(n) space.

        // @2022-08-03 03:49:43
        // The time complexity before was wrong and copilot did it ok,
        // because sort use some method to merge "chunks" (some youtube video said that)
        // and there are only two chunks. So the time complexity is O(\ln(n))

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
    pub fn find_median_sorted_arrays_non_recursive_recursion_pablo(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
    ) -> f64 {
        // impl of observation1
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

    #[allow(dead_code)]
    #[allow(unreachable_code, unused_variables)]
    pub fn find_median_sorted_arrays_binary_search_pablo(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // impl of observation2;
        panic!("too hard, i gave up, using answer from copilot");
        // in this case, nums1 should have more than `h` elements. So we swap nums1 and nums2 if nums1 is shorter.
        if nums1.len() + nums2.len() == 1 {
            return *nums1.get(0).unwrap_or(nums2.get(0).unwrap_or(&0)) as f64;
        }
        if nums1.len() < nums2.len() {
            return Solution::find_median_sorted_arrays_binary_search_pablo(nums2, nums1);
        }

        // extend num1 and nums2
        #[derive(Debug)]
        struct ExtendedVec<T> {
            vec: Vec<T>,
        }

        impl core::ops::Index<isize> for ExtendedVec<i32> {
            type Output = i32;
            // to solve the problem of empty list, add a -infty and a +infty to the list
            fn index(&self, ind: isize) -> &Self::Output {
                if ind <= 0 {
                    &i32::MIN
                } else if (ind as usize) < self.vec.len() + 1 {
                    &self.vec[ind as usize - 1]
                } else {
                    &i32::MAX
                }
            }
        }
        impl ExtendedVec<i32> {
            fn len(&self) -> isize {
                (self.vec.len() + 2) as isize
            }
            fn get(&self, ind: usize) -> Option<&i32> {
                self.vec.get(ind - 1)
            }
        }

        let nums1 = ExtendedVec { vec: nums1 };
        let nums2 = ExtendedVec { vec: nums2 };
        assert_eq!(nums1[-10 as isize], i32::MIN);

        println!("nums1: {:?}, nums2: {:?}", nums1, nums2);
        let l = nums1.len() + nums2.len();
        let h = if l % 2 == 0 { (l - 2) / 2 } else { (l - 1) / 2 };
        // sum of two index == h-1
        println!("l={}, h={}", l, h);

        let binary_search = || {
            let mut left = 0;
            let mut right = nums1.len() - 1;
            while left < right {
                let mid = (left + right) / 2;
                println!("left: {}, right: {}, mid: {}", left, right, mid);
                if nums1[mid] <= nums2[h - mid - 2] && nums1[mid + 1] >= nums2[h - mid - 1] {
                    return Some(mid);
                } else if nums1[mid] > nums2[h - mid] {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            if left == nums1.len() - 1 {
                None
            } else {
                Some(left)
            }
        };

        // let Some(pos) = binary_search();
        let remained_start1 = binary_search().unwrap() + 2;
        let remained_start2 = h - remained_start1;
        println!(
            "remained_start1: {}, remained_start2: {}",
            remained_start1, remained_start2
        );
        if l % 2 == 0 {
            let mut smallest_four = vec![
                nums1[remained_start1],
                nums2[remained_start2],
                nums1[remained_start1 + 1],
                nums2[remained_start2 + 1],
            ];
            smallest_four.sort();
            let smallest_two = &smallest_four[0..=1];
            println!("smallest_two: {:?}", smallest_two);
            (smallest_two[0] + smallest_two[1]) as f64 / 2.0
        } else {
            if nums1[remained_start1] < nums2[remained_start2] {
                nums1[remained_start1] as f64
            } else {
                nums2[remained_start2] as f64
            }
        }
    }

    /*
    The next answer was translated from this python code below:


    class Solution:
        def findMedianSortedArrays(self, nums1: List[int], nums2: List[int]) -> float:
            m, n = len(nums1), len(nums2)
            if m > n:
                nums1, nums2, m, n = nums2, nums1, n, m
            if n == 0:
                raise ValueError
            imin, imax, half_len = 0, m, (m + n + 1) // 2
            while imin <= imax:
                i = (imin + imax) // 2
                j = half_len - i
                if i < m and nums2[j - 1] > nums1[i]:
                    # i is too small, must increase it
                    imin = i + 1
                elif i > 0 and nums1[i - 1] > nums2[j]:
                    # i is too big, must decrease it
                    imax = i - 1
                else:
                    # i is perfect
                    if i == 0:
                        max_of_left = nums2[j - 1]
                    elif j == 0:
                        max_of_left = nums1[i - 1]
                    else:
                        max_of_left = max(nums1[i - 1], nums2[j - 1])
                    if (m + n) % 2 == 1:
                        return max_of_left
                    if i == m:
                        min_of_right = nums2[j]
                    elif j == n:
                        min_of_right = nums1[i]
                    else:
                        min_of_right = min(nums1[i], nums2[j])
                    return (max_of_left + min_of_right) / 2.0
            raise ValueError  # unreachable
             */

    #[allow(dead_code)]
    pub fn find_median_sorted_arrays_binary_search_copilot(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
    ) -> f64 {
        let len1 = nums1.len();
        let len2 = nums2.len();
        if len1 > len2 {
            return Solution::find_median_sorted_arrays_binary_search_copilot(nums2, nums1);
        }
        if len2 == 0 {
            panic!("two empty vectors");
        }
        // binary search on index of nums1 (the shorter one, to avoid overflow)
        let mut bs_min = 0;
        let mut bs_max = len1;
        let half_len = (len1 + len2 + 1) / 2;
        while bs_min <= bs_max {
            let bs_mid1 = (bs_min + bs_max) / 2;
            let bs_mid2 = half_len - bs_mid1;
            if bs_mid1 < len1 && nums2[bs_mid2 - 1] > nums1[bs_mid1] {
                // i is too small, increase it
                bs_min = bs_mid1 + 1;
            } else if bs_mid1 > 0 && nums1[bs_mid1 - 1] > nums2[bs_mid2] {
                // i is too big, decrease it
                bs_max = bs_mid1 - 1;
            } else {
                // i is perfect
                if bs_mid1 == 0 {
                    let max_of_left = nums2[bs_mid2 - 1];
                    if (len1 + len2) % 2 == 1 {
                        return max_of_left as f64;
                    }
                    if bs_mid2 == len2 {
                        let min_of_right = nums1[bs_mid1];
                        return (max_of_left + min_of_right) as f64 / 2.0;
                    } else {
                        let min_of_right = nums2[bs_mid2];
                        return (max_of_left + min_of_right) as f64 / 2.0;
                    }
                } else if bs_mid2 == 0 {
                    let max_of_left = nums1[bs_mid1 - 1];
                    if (len1 + len2) % 2 == 1 {
                        return max_of_left as f64;
                    }
                    if bs_mid1 == len1 {
                        let min_of_right = nums2[bs_mid2];
                        return (max_of_left + min_of_right) as f64 / 2.0;
                    } else {
                        let min_of_right = nums1[bs_mid1];
                        return (max_of_left + min_of_right) as f64 / 2.0;
                    }
                } else {
                    let max_of_left = std::cmp::max(nums1[bs_mid1 - 1], nums2[bs_mid2 - 1]);
                    if (len1 + len2) % 2 == 1 {
                        return max_of_left as f64;
                    }
                    if bs_mid1 == len1 {
                        let min_of_right = nums2[bs_mid2];
                        return (max_of_left + min_of_right) as f64 / 2.0;
                    } else {
                        let min_of_right = std::cmp::min(nums1[bs_mid1], nums2[bs_mid2]);
                        return (max_of_left + min_of_right) as f64 / 2.0;
                    }
                }
            }
        }
        panic!("unreachable");
    }
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        Solution::find_median_sorted_arrays_binary_search_copilot(nums1, nums2)
        // Solution::find_median_sorted_arrays_copilot(nums1, nums2)
    }
}
// @lc code=end

struct Solution;
pub fn main() {
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
        2.0
    );
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
        2.5
    );
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![0, 0, 0, 0, 0], vec![-1, 0, 0, 0, 0, 0, 1]),
        0.0
    );
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![], vec![1, 2, 3, 4]),
        2.5
    );
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1], vec![2, 3, 4, 5]),
        3.0
    );
    assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1]), 1.0);
    assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![2, 3]), 2.5);
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![3], vec![1, 2, 4]),
        1.0
    ); // 2042
}
