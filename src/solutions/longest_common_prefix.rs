// 14. Longest Common Prefix
// https://leetcode.com/problems/longest-common-prefix/

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut result = String::with_capacity(strs[0].len());

    for i in 0..strs[0].len() {
        for j in 1..strs.len() {
            if i >= strs[j].len()
                || strs[j].as_bytes()[i] != strs[0].as_bytes()[i]
            {
                return result;
            }
        }
        result.push(strs[0].as_bytes()[i] as char);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(
            longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl"
        );
        assert_eq!(
            longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            ""
        );
    }
    
    #[test]
    fn test_single_string() {
        assert_eq!(longest_common_prefix(vec!["hello".to_string()]), "hello");
    }
    
    #[test]
    fn test_identical_strings() {
        assert_eq!(
            longest_common_prefix(vec!["test".to_string(), "test".to_string(), "test".to_string()]),
            "test"
        );
    }
}
