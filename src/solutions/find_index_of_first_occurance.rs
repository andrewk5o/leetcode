// 28. Find the Index of the First Occurrence in a String
// https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string

pub fn str_str(haystack: String, needle: String) -> i32 {
    let haystack = haystack.as_bytes();
    let needle = needle.as_bytes();

    if needle.len() > haystack.len() {
        return -1;
    }

    for i in 0..=(haystack.len() - needle.len()) {
        if haystack[i..i + needle.len()] == *needle {
            return i as i32;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_str_str() {
        assert_eq!(str_str("hello".to_string(), "ll".to_string()), 2);
        assert_eq!(str_str("aaaaa".to_string(), "bba".to_string()), -1);
    }
}
