#![allow(dead_code)]

use std::collections::HashMap;

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

    /// # 1784. 检查二进制字符串字段
    ///
    /// https://leetcode.cn/problems/check-if-binary-string-has-at-most-one-segment-of-ones/
    pub fn check_ones_segment(s: String) -> bool {
        let s = s.into_bytes();
        for i in 0..s.len() - 1 {
            if s[i] == b'0' && s[i + 1] == b'1' {
                return false;
            }
        }
        true
    }

    /// # 921. 使括号有效的最少添加
    ///
    /// https://leetcode.cn/problems/minimum-add-to-make-parentheses-valid/
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut result = 0;
        let mut lp = 0;
        for c in s.as_bytes() {
            if *c == b'(' {
                lp += 1;
            } else if lp > 0 {
                lp -= 1;
            } else {
                result += 1;
            }
        }
        result + lp
    }

    /// # 811. 子域名访问计数
    ///
    /// https://leetcode.cn/problems/subdomain-visit-count/
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut counter: HashMap<&str, usize> = HashMap::new();
        for s in cpdomains.iter() {
            let rep: usize;
            let mut url: &str;
            if let Some(whitespace) = s.find(' ') {
                rep = s[..whitespace].parse().unwrap();
                url = &s[whitespace + 1..];
            } else {
                unreachable!()
            }
            let e = counter.entry(url).or_default();
            *e += rep;
            while let Some(pos) = url.find('.') {
                let ss = pos + 1;
                let e = counter.entry(&url[ss..]).or_default();
                *e += rep;
                url = &url[ss..];
            }
        }
        let mut result = Vec::with_capacity(counter.len());
        for (k, v) in counter.into_iter() {
            result.push(format!("{} {}", v, k));
        }
        result
    }

    /// # 927. 三等分
    ///
    /// https://leetcode.cn/problems/three-equal-parts/
    pub fn three_equal_parts(arr: Vec<i32>) -> Vec<i32> {
        let n = arr.len();
        let s: i32 = arr.iter().sum();
        if s % 3 != 0 {
            return vec![-1, -1];
        }
        if s == 0 {
            return vec![0, 2]; // [0,0,0]
        }
        let group_ones = s / 3;
        let mut ones = 0;
        let mut fir1 = 0;
        let mut sec1 = 0;
        let mut thi1 = 0;
        for i in 0..n {
            if arr[i] == 1 {
                if ones == 0 {
                    fir1 = i;
                }
                if ones == group_ones {
                    sec1 = i;
                }
                if ones == group_ones * 2 {
                    thi1 = i;
                    break;
                }
                ones += 1;
            }
        }
        let group_len = n - thi1;
        if fir1 + group_len <= sec1 && sec1 + group_len <= thi1 {
            for i in 0..group_len {
                if arr[fir1 + i] != arr[sec1 + i] || arr[fir1 + i] != arr[thi1 + i] {
                    return vec![-1, -1];
                }
            }
            return vec![(fir1 + group_len - 1) as i32, (sec1 + group_len) as i32];
        }

        vec![-1, -1]
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

    #[test]
    fn check_ones_segment() {
        assert_eq!(Solution::check_ones_segment("1001".to_string()), false);
        assert_eq!(Solution::check_ones_segment("110".to_string()), true);
        assert_eq!(Solution::check_ones_segment("1000".to_string()), true);
        assert_eq!(Solution::check_ones_segment("111".to_string()), true);
        assert_eq!(Solution::check_ones_segment("110111".to_string()), false);
    }

    #[test]
    fn min_add_to_make_valid() {
        assert_eq!(Solution::min_add_to_make_valid("(((".to_string()), 3);
        assert_eq!(Solution::min_add_to_make_valid("())".to_string()), 1);
    }

    #[test]
    fn subdomain_visits() {
        let mut get = Solution::subdomain_visits(vec!["9001 discuss.leetcode.com".to_string()]);
        let mut want = vec![
            "9001 leetcode.com".to_string(),
            "9001 discuss.leetcode.com".to_string(),
            "9001 com".to_string(),
        ];
        get.sort();
        want.sort();
        assert_eq!(get, want);

        let mut get = Solution::subdomain_visits(vec![
            "900 google.mail.com".to_string(),
            "50 yahoo.com".to_string(),
            "1 intel.mail.com".to_string(),
            "5 wiki.org".to_string(),
        ]);
        let mut want = vec![
            "901 mail.com".to_string(),
            "50 yahoo.com".to_string(),
            "900 google.mail.com".to_string(),
            "5 wiki.org".to_string(),
            "5 org".to_string(),
            "1 intel.mail.com".to_string(),
            "951 com".to_string(),
        ];
        get.sort();
        want.sort();
        assert_eq!(get, want);
    }

    #[test]
    fn three_equal_parts() {
        assert_eq!(Solution::three_equal_parts(vec![1, 0, 1, 0, 1]), vec![0, 3]);
        assert_eq!(Solution::three_equal_parts(vec![1, 1, 0, 0, 1]), vec![0, 2]);
        assert_eq!(
            Solution::three_equal_parts(vec![1, 1, 0, 1, 1]),
            vec![-1, -1]
        );
        assert_eq!(
            Solution::three_equal_parts(vec![0, 1, 0, 1, 1, 0, 1, 1, 0, 1]),
            vec![3, 7]
        );
    }
}
