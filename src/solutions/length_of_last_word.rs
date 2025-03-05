// 58. Length of Last Word
// https://leetcode.com/problems/length-of-last-word/

pub fn length_of_last_word(s: String) -> i32 {
    let s_bytes = s.as_bytes();
    let mut k: usize = 0;

    for i in (0..s_bytes.len()).rev() {
        if s_bytes[i] != b' ' {
            k += 1;
        } else if k > 0 {
            return k as i32;
        }
    }
    k as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_last_word() {
        assert_eq!(length_of_last_word("Hello World".to_string()), 5);
        assert_eq!(
            length_of_last_word("   fly me   to   the moon  ".to_string()),
            4
        );
    }

    #[test]
    fn test_single_word() {
        assert_eq!(length_of_last_word("Hello".to_string()), 5);
    }
    
    #[test]
    fn test_trailing_spaces() {
        assert_eq!(length_of_last_word("Hello ".to_string()), 5);
    }
}
