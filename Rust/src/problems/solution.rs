#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use super::{Solution, TreeNode};

impl Solution {
    /// # 124. 二叉树中的最大路径和
    ///
    /// https://leetcode.cn/problems/binary-tree-maximum-path-sum/
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = i32::MIN;
        Self::max_contri(root, &mut result);
        result
    }

    fn max_contri(node: Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
        if let Some(node) = node {
            let left = Self::max_contri(node.borrow().left.clone(), max).max(0);
            let right = Self::max_contri(node.borrow().right.clone(), max).max(0);

            let within = node.borrow().val + left + right;
            let new_max = within.max(*max);
            *max = new_max;

            node.borrow().val + left.max(right)
        } else {
            0
        }
    }

    fn max_sum_of_subsequence(arr: Vec<i32>) -> i32 {
        let mut max_subs_sums = arr.clone();
        for i in 0..arr.len() {
            for j in 0..i {
                if arr[i] > arr[j] && max_subs_sums[i] < max_subs_sums[j] + arr[i] {
                    max_subs_sums[i] = max_subs_sums[j] + arr[i];
                }
            }
        }

        max_subs_sums.into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::{Solution, TreeNode};

    #[test]
    fn max_sum_of_subsequence() {
        assert_eq!(
            Solution::max_sum_of_subsequence(vec![1, 100, 1, 2, 99, 4, 5]),
            102
        );
        assert_eq!(Solution::max_sum_of_subsequence(vec![1 + 2 + 3 + 100]), 106);
        assert_eq!(Solution::max_sum_of_subsequence(vec![10, 5, 3]), 10);
        assert_eq!(Solution::max_sum_of_subsequence(vec![1]), 1);
    }

    #[test]
    fn max_path_sum() {
        let tree = TreeNode::from(vec![
            Some(-10),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        assert_eq!(Solution::max_path_sum(tree), 42);

        let tree = TreeNode::from(vec![Some(1), Some(2), Some(3)]);
        assert_eq!(Solution::max_path_sum(tree), 6);

        let tree = TreeNode::from(vec![Some(-3)]);
        assert_eq!(Solution::max_path_sum(tree), -3);
    }
}
