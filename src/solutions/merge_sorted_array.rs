// 88. Merge Sorted Array
// https://leetcode.com/problems/merge-sorted-array/

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut m = m - 1;
    let mut n = n - 1;
    let mut k = m + n + 1;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }
}
