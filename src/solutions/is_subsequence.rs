// 392. Is Subsequence

pub fn is_subsequence(s: String, t: String) -> bool {
    let s_bytes = s.as_bytes();
    let t_bytes = t.as_bytes();

    let mut k: usize = 0;

    for c in t_bytes {
        if k == s.len() {
            break;
        }

        if *c == s_bytes[k] {
            k += 1;
        }
    }

    k == s.len()
}

#[cfg(test)]
mod tests {
    use super::is_subsequence;

    #[test]
    fn test_is_subsequence_true() {
        assert_eq!(
            is_subsequence("abc".to_string(), "ahbgdc".to_string()),
            true
        );
    }

    #[test]
    fn test_is_subsequence_false() {
        assert_eq!(
            is_subsequence("axc".to_string(), "ahbgdc".to_string()),
            false
        );
    }
}
