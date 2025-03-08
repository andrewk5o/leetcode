// 50. Pow(x, n)
// https://leetcode.com/problems/powx-n/

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_pow() {
        let res1 = my_pow(2.0, 10);
        assert_eq!(res1, 1024.0);

        let res2 = my_pow(2.0, -2);
        assert!((res2 - 0.25).abs() < 1e-6);
    }
}
