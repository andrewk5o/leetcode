// 80. Remove Duplicates from Sorted Array II
// https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut k: usize = 1;
    let mut count = 1;
    for n in 1..nums.len() {
        if nums[n] != nums[n - 1] {
            nums[k] = nums[n];
            k += 1;
            count = 1;
        } else if count < 2 {
            nums[k] = nums[n];
            k += 1;
            count += 1;
        }
    }
    k as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(remove_duplicates(&mut nums), 5);
        assert_eq!(&nums[..5], &[1, 1, 2, 2, 3]);

        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        assert_eq!(remove_duplicates(&mut nums), 7);
        assert_eq!(&nums[..7], &[0, 0, 1, 1, 2, 3, 3]);
    }

    #[test]
    fn test_single_element() {
        let mut nums = vec![1];
        assert_eq!(remove_duplicates(&mut nums), 1);
        assert_eq!(&nums[..1], &[1]);
    }

    #[test]
    fn test_no_duplicates() {
        let mut nums = vec![1, 2, 3, 4];
        assert_eq!(remove_duplicates(&mut nums), 4);
        assert_eq!(&nums[..4], &[1, 2, 3, 4]);
    }
}
