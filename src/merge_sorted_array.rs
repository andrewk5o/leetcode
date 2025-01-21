// 88. Merge Sorted Array
pub struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut m = usize::try_from(m).unwrap();
        let mut n = usize::try_from(n).unwrap();
        let mut k = usize::try_from(m + n - 1).unwrap();

        while m > 0 && n > 0 {
            if nums1[m - 1] > nums2[n - 1] {
                nums1[k] = nums1[m - 1];
                m -= 1;
            } else {
                nums1[k] = nums2[n - 1];
                n -= 1;
            }
            k -= 1;
        }

        while n > 0 {
            nums1[k] = nums2[n - 1];
            n -= 1;
            k -= 1;
        }
    }
}
