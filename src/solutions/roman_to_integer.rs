use std::collections::HashMap;

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

    for i in 0..s.len() {
        if i + 1 < s.len() && roman[&s[i]] < roman[&s[i + 1]] {
            res -= roman[&s[i]];
        } else {
            res += roman[&s[i]];
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_int() {
        assert_eq!(roman_to_int("III".to_string()), 3);
        assert_eq!(roman_to_int("IV".to_string()), 4);
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
