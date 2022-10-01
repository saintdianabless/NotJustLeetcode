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
}

#[cfg(test)]
mod tests {
    use crate::problems::{Solution, TreeNode};

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
