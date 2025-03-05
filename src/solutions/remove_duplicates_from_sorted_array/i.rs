// 26. Remove duplicates from sorted array

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut k: usize = 1;
    for n in 1..nums.len() {
        if nums[n] != nums[n - 1] {
            nums[k] = nums[n];
            k += 1;
        }
    }
    i32::try_from(k).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(remove_duplicates(&mut nums), 2);
        assert_eq!(&nums[..2], &[1, 2]);

        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(remove_duplicates(&mut nums), 5);
        assert_eq!(&nums[..5], &[0, 1, 2, 3, 4]);
    }
}
