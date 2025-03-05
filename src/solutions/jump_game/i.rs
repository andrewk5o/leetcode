// 55. Jump Game

pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut goal = nums.len() - 1;

    for i in (0..nums.len() - 1).rev() {
        if i + nums[i] as usize >= goal {
            goal = i;
        };
    }

    goal == 0
}

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
    fn test_can_jump() {
        assert_eq!(can_jump(vec![2, 3, 1, 1, 4]), true);
        assert_eq!(can_jump(vec![3, 2, 1, 0, 4]), false);
    }

    #[test]
    fn test_length_of_last_word() {
        assert_eq!(length_of_last_word("Hello World".to_string()), 5);
        assert_eq!(
            length_of_last_word("   fly me   to   the moon  ".to_string()),
            4
        );
    }

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
}
