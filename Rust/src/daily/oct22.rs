#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use super::{ListNode, Solution};

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

    /// # 1800. 最大升序子数组和
    ///
    /// https://leetcode.cn/problems/maximum-ascending-subarray-sum/
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut max_sub_sum = nums[0];
        let mut cur_sum = nums[0];
        for i in 1..nums.len() {
            if nums[i] <= nums[i - 1] {
                max_sub_sum = max_sub_sum.max(cur_sum);
                cur_sum = nums[i];
            } else {
                cur_sum += nums[i];
            }
        }
        max_sub_sum.max(cur_sum)
    }

    /// # 870. 优势洗牌
    ///
    /// https://leetcode.cn/problems/advantage-shuffle/
    pub fn advantage_count(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let n = nums1.len();
        let mut result = Vec::with_capacity(n);
        result.resize(n, 0);

        let mut index1: Vec<usize> = (0..n).collect();
        let mut index2: Vec<usize> = index1.clone();
        index1.sort_by(|&a, &b| (nums1[a]).cmp(&nums1[b]));
        index2.sort_by(|&a, &b| (nums2[a]).cmp(&nums2[b]));

        let mut l = 0;
        let mut r = n - 1;
        for i in 0..n {
            if nums1[index1[i]] > nums2[index2[l]] {
                result[index2[l]] = nums1[index1[i]];
                l += 1;
            } else {
                result[index2[r]] = nums1[index1[i]];
                r -= 1;
            }
        }

        result
    }

    /// # 856. 括号的分数
    ///
    /// https://leetcode.cn/problems/score-of-parentheses/
    pub fn score_of_parentheses(s: String) -> i32 {
        let s = s.into_bytes();
        let mut result = 0;
        let mut depth = 0;
        for i in 0..s.len() {
            depth += if s[i] == b'(' { 1 } else { -1 };
            if s[i] == b')' && s[i - 1] == b'(' {
                result += 1 << depth;
            }
        }
        result
    }

    /// # 801. 使序列递增的最小交换次数
    ///
    /// https://leetcode.cn/problems/minimum-swaps-to-make-sequences-increasing/
    pub fn min_swap(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let mut a = 0;
        let mut b = 1;
        for i in 1..n {
            let last_a = a;
            let last_b = b;
            a = n;
            b = n;
            if nums1[i] > nums1[i - 1] && nums2[i] > nums2[i - 1] {
                a = a.min(last_a);
                b = b.min(last_b + 1);
            }
            if nums1[i] > nums2[i - 1] && nums2[i] > nums1[i - 1] {
                a = a.min(last_b); // 上次交换了，这次直接就符合
                b = b.min(last_a + 1); // 上次没交换，这次需要交换
            }
        }
        a.min(b) as i32
    }

    /// # 1790. 仅执行一次字符串交换能否使两个字符串相等
    ///
    /// https://leetcode.cn/problems/check-if-one-string-swap-can-make-strings-equal/
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let s1 = s1.into_bytes();
        let s2 = s2.into_bytes();
        let mut a = -1;
        let mut b = -1;
        for i in 0..s1.len() {
            if s1[i] != s2[i] {
                if a < 0 {
                    a = i as i32;
                } else if b < 0 {
                    b = i as i32;
                } else {
                    return false;
                }
            }
        }
        a < 0 || (b > 0 && s1[a as usize] == s2[b as usize] && s1[b as usize] == s2[a as usize])
    }

    /// # 817. 链表组件
    ///
    /// https://leetcode.cn/problems/linked-list-components/
    pub fn num_components(head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32 {
        let mut head = head;
        let mut result = 0;
        let mut f = false;
        let nums = nums.into_iter().collect::<HashSet<i32>>();
        while let Some(node) = head {
            if !nums.contains(&node.val) {
                f = false;
            } else if !f {
                f = true;
                result += 1;
            }
            head = node.next;
        }
        result
    }

    /// # 769. 最多能完成排序的块
    ///
    /// https://leetcode.cn/problems/max-chunks-to-make-sorted/
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut max_so_far = arr[0];
        for i in 0..arr.len() {
            max_so_far = max_so_far.max(arr[i]);
            if max_so_far == i as i32 {
                result += 1;
            }
        }
        result
    }

    /// # 940. 不同的子序列 II
    ///
    /// https://leetcode.cn/problems/distinct-subsequences-ii/
    pub fn distinct_subseq_ii(s: String) -> i32 {
        let r#mod = 1e9 as i32 + 7;
        let mut total = 0;
        let mut dp = [0; 26];

        for c in s.into_bytes().into_iter() {
            let p = (c - b'a') as usize;
            let prev = dp[p];
            dp[p] = (total + 1) % r#mod;
            total = ((total - prev + dp[p]) % r#mod + r#mod) % r#mod;
        }

        total
    }

    /// # 1441. 用栈操作构建数组
    ///
    /// https://leetcode.cn/problems/build-an-array-with-stack-operations/
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut result = vec![];
        let mut prev = 0;
        for n in target {
            for _ in 0..(n - prev - 1) {
                result.push("Push".to_string());
                result.push("Pop".to_string());
            }
            result.push("Push".to_string());
            prev = n;
        }
        result
    }

    /// # 886. 可能的二分法
    ///
    /// https://leetcode.cn/problems/possible-bipartition/
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        let mut norm = (0..n).map(|_| Vec::new()).collect::<Vec<_>>();
        for v in dislikes {
            let a = v[0] as usize;
            let b = v[1] as usize;
            norm[a - 1].push(b - 1);
            norm[b - 1].push(a - 1);
        }
        let mut gs = (0..n).map(|_| 0).collect::<Vec<_>>();

        for i in 0..n {
            if gs[i] == 0 && !Solution::possible_bipartition_dfs(i, 1, &mut gs, &norm) {
                return false;
            }
        }

        true
    }

    fn possible_bipartition_dfs(
        cur: usize,
        g: i32,
        gs: &mut Vec<i32>,
        norm: &Vec<Vec<usize>>,
    ) -> bool {
        gs[cur] = g;
        for op in norm[cur].iter() {
            let op = *op;
            if gs[op] != 0 && gs[op] == gs[cur] {
                // 对方已经分组并且分组与自己相同
                return false;
            }
            if gs[op] == 0 && !Solution::possible_bipartition_dfs(op, 3 ^ g, gs, norm) {
                // 对方未分组，给它不同于cur的分组(3 ^ g)
                return false;
            }
        }

        true
    }

    /// # 904. 水果成篮
    ///
    /// https://leetcode.cn/problems/fruit-into-baskets/solution/
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut count: HashMap<i32, usize> = HashMap::new();
        let mut result = 0;
        let mut l = 0;

        for r in 0..fruits.len() {
            let f = fruits[r];
            count.entry(f).and_modify(|o| *o += 1).or_insert(1);
            while count.keys().len() > 2 {
                let lf = fruits[l];
                let e = count.entry(lf).and_modify(|o| *o -= 1).or_insert(0);
                if *e == 0 {
                    count.remove(&lf);
                }
                l += 1;
            }
            result = result.max(r - l + 1);
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn total_fruit() {
        assert_eq!(Solution::total_fruit(vec![1, 2, 1]), 3);
        assert_eq!(Solution::total_fruit(vec![0, 1, 2, 2]), 3);
        assert_eq!(Solution::total_fruit(vec![1, 2, 3, 2, 2]), 4);
        assert_eq!(
            Solution::total_fruit(vec![3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4]),
            5
        );
    }

    #[test]
    fn possible_bipartition() {
        assert_eq!(
            Solution::possible_bipartition(4, vec![vec![1, 2], vec![1, 3], vec![2, 4]]),
            true
        );
        assert_eq!(
            Solution::possible_bipartition(3, vec![vec![1, 2], vec![1, 3], vec![2, 3]]),
            false
        );
        assert_eq!(
            Solution::possible_bipartition(
                5,
                vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![1, 5]]
            ),
            false
        );
    }

    #[test]
    fn build_array() {
        assert_eq!(
            Solution::build_array(vec![1, 3], 3),
            vec![
                "Push".to_string(),
                "Push".to_string(),
                "Pop".to_string(),
                "Push".to_string()
            ]
        );
        assert_eq!(
            Solution::build_array(vec![1, 2, 3], 3),
            vec!["Push".to_string(), "Push".to_string(), "Push".to_string()]
        );
        assert_eq!(
            Solution::build_array(vec![1, 2], 4),
            vec!["Push".to_string(), "Push".to_string()]
        );
    }

    #[test]
    fn distinct_subseq_ii() {
        assert_eq!(Solution::distinct_subseq_ii("abc".to_string()), 7);
        assert_eq!(Solution::distinct_subseq_ii("aba".to_string()), 6);
        assert_eq!(Solution::distinct_subseq_ii("aaa".to_string()), 3);
    }

    #[test]
    fn max_chunks_to_sorted() {
        assert_eq!(Solution::max_chunks_to_sorted(vec![4, 3, 2, 1, 0]), 1);
        assert_eq!(Solution::max_chunks_to_sorted(vec![1, 0, 2, 3, 4]), 4);
        assert_eq!(Solution::max_chunks_to_sorted(vec![0]), 1);
    }

    #[test]
    fn num_components() {
        // let lst = ListNode::from_vec(vec![1,2,3,4,5,6]);
        // println!("{:#?}", lst);
        assert_eq!(
            Solution::num_components(ListNode::from_vec(vec![0, 1, 2, 3]), vec![0, 1, 3]),
            2
        );
        assert_eq!(
            Solution::num_components(ListNode::from_vec(vec![0, 1, 2, 3, 4]), vec![0, 3, 1, 4]),
            2
        );
    }

    #[test]
    fn are_almost_equal() {
        assert_eq!(
            Solution::are_almost_equal("aabbcc".to_string(), "abbacc".to_string()),
            true
        );
        assert_eq!(
            Solution::are_almost_equal("bank".to_string(), "kanb".to_string()),
            true
        );
        assert_eq!(
            Solution::are_almost_equal("attack".to_string(), "defend".to_string()),
            false
        );
        assert_eq!(
            Solution::are_almost_equal("kelb".to_string(), "kelb".to_string()),
            true
        );
        assert_eq!(
            Solution::are_almost_equal("abcd".to_string(), "xycd".to_string()),
            false
        );
    }

    #[test]
    fn min_swap() {
        assert_eq!(Solution::min_swap(vec![1, 3, 5, 4], vec![1, 2, 3, 7]), 1);
        assert_eq!(
            Solution::min_swap(vec![0, 3, 5, 8, 9], vec![2, 1, 4, 6, 9]),
            1
        );
    }

    #[test]
    fn score_of_parentheses() {
        assert_eq!(Solution::score_of_parentheses("()".to_string()), 1);
        assert_eq!(Solution::score_of_parentheses("(())".to_string()), 2);
        assert_eq!(Solution::score_of_parentheses("()()".to_string()), 2);
        assert_eq!(Solution::score_of_parentheses("(()(()))".to_string()), 6);
    }

    #[test]
    fn advantage_count() {
        assert_eq!(
            Solution::advantage_count(vec![2, 7, 11, 15], vec![1, 10, 4, 11]),
            vec![2, 11, 7, 15]
        );
        assert_eq!(
            Solution::advantage_count(vec![12, 24, 8, 32], vec![13, 25, 32, 11]),
            vec![24, 32, 8, 12]
        );
    }

    #[test]
    fn max_ascending_sum() {
        assert_eq!(Solution::max_ascending_sum(vec![10, 20, 30, 5, 10, 50]), 65);
        assert_eq!(Solution::max_ascending_sum(vec![10, 20, 30, 40, 50]), 150);
        assert_eq!(
            Solution::max_ascending_sum(vec![12, 17, 15, 13, 10, 11, 12]),
            33
        );
        assert_eq!(Solution::max_ascending_sum(vec![100, 10, 1]), 100);
        assert_eq!(
            Solution::max_ascending_sum(vec![100, 1, 1000, 1, 10000]),
            10001
        );
    }

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
