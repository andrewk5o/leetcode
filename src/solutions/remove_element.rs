// 27. Remove element

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut k = 0;
    for i in 0..nums.len() {
        if nums[i] != val {
            nums[k] = nums[i];
            k += 1;
        }
    }
    k as i32
}

pub fn str_str(haystack: String, needle: String) -> i32 {
    let haystack = haystack.as_bytes();
    let needle = needle.as_bytes();
    let mut k = 0;

    for i in 0..haystack.len() {
        if haystack[i] == needle[k] {
            k += 1;
        } else {
            k = 0;
        }
        if k > needle.len() - 1 {
            return (i - k + 1) as i32;
        }
    }
    -1
}
