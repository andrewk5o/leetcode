// 88. Merge Sorted Array
pub struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = usize::try_from(m).unwrap();
        let mut j = usize::try_from(n).unwrap();
        let mut k = usize::try_from(m + n - 1).unwrap();

        while i > 0 && j > 0 {
            if nums1[i - 1] > nums2[j - 1] {
                nums1[k] = nums1[i - 1];
                i -= 1;
            } else {
                nums1[k] = nums2[j - 1];
                j -= 1;
            }
            k -= 1;
        }

        while j > 0 {
            nums1[k] = nums2[j - 1];
            j -= 1;
            k -= 1;
        }
    }
}
