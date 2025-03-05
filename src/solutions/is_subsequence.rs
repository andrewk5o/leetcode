// 392. Is Subsequence

pub fn is_subsequence(s: String, t: String) -> bool {
    let s_bytes = s.as_bytes();
    let t_bytes = t.as_bytes();

    let mut k: usize = 0;

    for c in t_bytes {
        if k == s.len() {
            return true;
        }

        if *c == s_bytes[k] {
            k += 1;
        }
    }

    false
}
