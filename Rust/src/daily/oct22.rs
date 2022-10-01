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
}

#[cfg(test)]
mod test {
    use super::*;

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
