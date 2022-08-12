#
# @lc app=leetcode id=4 lang=python3
#
# [4] Median of Two Sorted Arrays
#

# @lc code=start
from typing import List


class Solution:
    def findMedianSortedArrays(self, nums1: List[int], nums2: List[int]) -> float:
        len1, len2 = len(nums1), len(nums2)
        if len1 > len2:
            # nums1, nums2, len1, len2 = nums2, nums1, len2, len1
            return self.findMedianSortedArrays(nums1=nums2, nums2=nums1)
        if len2 == 0:
            raise ValueError

        bs_min, bs_max, half_len = 0, len1, (len1 + len2 + 1) // 2
        while bs_min <= bs_max:
            mid1 = (bs_min + bs_max) // 2
            mid2 = half_len - mid1
            if mid1 < len1 and nums2[mid2 - 1] > nums1[mid1]:
                bs_min = mid1 + 1
            elif mid1 > 0 and nums1[mid1 - 1] > nums2[mid2]:
                bs_max = mid1 - 1
            else:
                if mid1 == 0:
                    max_of_left = nums2[mid2 - 1]
                elif mid2 == 0:
                    max_of_left = nums1[mid1 - 1]
                else:
                    max_of_left = max(nums1[mid1 - 1], nums2[mid2 - 1])

                if (len1 + len2) % 2 == 1:
                    return max_of_left

                if mid1 == len1:
                    min_of_right = nums2[mid2]
                elif mid2 == len2:
                    min_of_right = nums1[mid1]
                else:
                    min_of_right = min(nums1[mid1], nums2[mid2])

                return (max_of_left + min_of_right) / 2.0
        raise ValueError


# @lc code=end

if __name__ == "__main__":
    assert Solution().findMedianSortedArrays([1, 3], [2]) == 2.0
    assert Solution().findMedianSortedArrays([1, 2], [3, 4]) == 2.5
    assert (
        Solution().findMedianSortedArrays([0, 0, 0, 0, 0], [-1, 0, 0, 0, 0, 0, 1])
        == 0.0
    )
    assert Solution().findMedianSortedArrays([], [1, 2, 3, 4]) == 2.5
    assert Solution().findMedianSortedArrays([1], [2, 3, 4, 5]) == 3.0
    assert Solution().findMedianSortedArrays([], [1]) == 1.0
    assert Solution().findMedianSortedArrays([], [2, 3]) == 2.5
    assert Solution().findMedianSortedArrays([3], [1, 2, 4]) == 2.5
