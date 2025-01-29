// 88. Merge Sorted Array

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let (mut m, mut n, mut k) = (m - 1, n - 1, m + n - 1);

    while m >= 0 && n >= 0 {
        if nums1[m as usize] > nums2[n as usize] {
            nums1[k as usize] = nums1[m as usize];
            m -= 1;
        } else {
            nums1[k as usize] = nums2[n as usize];
            n -= 1;
        }
        k -= 1;
    }

    while n >= 0 {
        nums1[k as usize] = nums2[n as usize];
        n -= 1;
        k -= 1;
    }
}
