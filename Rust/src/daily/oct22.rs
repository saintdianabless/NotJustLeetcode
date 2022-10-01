#![allow(dead_code)]

use super::Solution;

impl Solution {
    /// # 1694. 重新格式化电话号码
    ///
    /// https://leetcode.cn/problems/reformat-phone-number/
    pub fn reformat_number(number: String) -> String {
        let mut digits = number.into_bytes();
        digits.retain(|c| c.is_ascii_digit());
        let digits = unsafe { String::from_utf8_unchecked(digits) };
        let mut n = digits.len();
        let mut result = String::with_capacity(n + n / 3);
        let mut i = 0;
        while n != 0 {
            if n > 4 {
                result.push_str(&digits[i..i + 3]);
                result.push('-');
                i += 3;
                n -= 3;
            } else {
                if n == 4 {
                    result.push_str(&digits[i..i + 2]);
                    result.push('-');
                    result.push_str(&digits[i + 2..]);
                } else {
                    result.push_str(&digits[i..])
                }
                n = 0
            }
        }
        result
    }

    /// # 777. 在LR字符串中交换相邻字符
    ///
    /// https://leetcode.cn/problems/swap-adjacent-in-lr-string/
    pub fn can_transform(start: String, end: String) -> bool {
        let start = start.as_bytes();
        let end = end.as_bytes();
        let mut l = 0;
        let mut r = 0;
        let m = start.len();
        let n = end.len();
        while l != m && r != n {
            while l != m && start[l] == b'X' {
                l += 1;
            }
            while r != n && end[r] == b'X' {
                r += 1;
            }
            if l != m && r != n {
                if start[l] != end[r] {
                    return false;
                }
                if start[l] == b'L' && l < r {
                    return false;
                }
                if start[l] == b'R' && l > r {
                    return false;
                }
                l += 1;
                r += 1;
            }
        }
        while l < m {
            if start[l] != b'X' {
                return false;
            }
            l += 1;
        }
        while r < n {
            if end[r] != b'X' {
                return false;
            }
            r += 1;
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_transform() {
        assert!(Solution::can_transform(
            "RXXLRXRXL".to_string(),
            "XRLXXRRLX".to_string()
        ));
        assert!(Solution::can_transform(
            "RXXLRXRXL".to_string(),
            "XRLXXRXRL".to_string()
        ));
    }

    #[test]
    fn reformat_number() {
        assert_eq!(
            Solution::reformat_number("1-23-45 6".to_string()),
            "123-456".to_string()
        );
        assert_eq!(
            Solution::reformat_number("123 4-567".to_string()),
            "123-45-67".to_string()
        );
        assert_eq!(
            Solution::reformat_number("123 4-5678".to_string()),
            "123-456-78".to_string()
        );
        assert_eq!(
            Solution::reformat_number("12".to_string()),
            "12".to_string()
        );
        assert_eq!(
            Solution::reformat_number("--17-5 229 35-39475 ".to_string()),
            "175-229-353-94-75".to_string()
        );
    }
}
