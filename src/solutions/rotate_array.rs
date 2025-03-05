// 189. Rotate array

use std::collections::{HashMap, HashSet};

fn swap(nums: &mut Vec<i32>, l: i32, r: i32) {
    let (l, r) = (l as usize, r as usize);
    (nums[l], nums[r]) = (nums[l], nums[r]);
}

pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let length = nums.len() as i32;

    let k = k % length;

    let (mut l, mut r) = (0, length - 1);

    while l < r {
        swap(nums, l, r);
        l += 1;
        r -= 1;
    }

    (l, r) = (0, k - 1);
    while l < r {
        swap(nums, l, r);
        l += 1;
        r -= 1;
    }

    (l, r) = (k, length - 1);
    while l < r {
        swap(nums, l, r);
        l += 1;
        r -= 1;
    }
}

pub fn str_str(haystack: String, needle: String) -> i32 {
    let haystack = haystack.as_bytes();
    let needle = needle.as_bytes();

    if needle.len() > haystack.len() {
        return -1 as i32;
    }

    for i in 0..=(haystack.len() - needle.len()) {
        if haystack[i..i + needle.len()] == *needle {
            return i as i32;
        }
    }
    -1
}

pub fn length_of_longest_substring(s: String) -> i32 {
    let s = &s.as_bytes();

    let mut sub_str = HashSet::<u8>::with_capacity(s.len());
    let (mut l, mut r, mut max) = (0, 0, 0);

    while r < s.len() {
        if sub_str.contains(&s[r]) {
            sub_str.remove(&s[l]);
            l += 1;
        } else {
            sub_str.insert(s[r]);
            r += 1;
            max = max.max(r - l);
        }
    }

    max as i32
}

pub fn my_pow(x: f64, n: i32) -> f64 {
    fn pow(x: f64, n: i32) -> f64 {
        if x == 0.0 {
            return 0.0;
        }
        if n == 0 {
            return 1.0;
        }

        let res = pow(x * x, n / 2);

        return res * if n % 2 == 0 { 1.0 } else { x };
    }

    let res = pow(x, n.abs());

    return if n > 0 { res } else { 1.0 / res };
}

pub fn roman_to_int(s: String) -> i32 {
    let s = s.as_bytes();

    let roman = HashMap::<u8, i32>::from([
        (b'I', 1),
        (b'V', 5),
        (b'X', 10),
        (b'L', 50),
        (b'C', 100),
        (b'D', 500),
        (b'M', 1000),
    ]);

    let mut res = 0;

    for i in (0..s.len()).rev() {
        if roman[&s[i]] < roman[&s[i + 1]] {
            res -= roman[&s[i]];
        } else {
            res += roman[&s[i]];
        }
    }

    res
}
