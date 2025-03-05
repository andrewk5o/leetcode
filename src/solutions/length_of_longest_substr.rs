// 3. Longest Substring Without Repeating Characters
// https://leetcode.com/problems/longest-substring-without-repeating-characters/

use std::collections::HashSet;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
    }
}
