#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

mod solution;

struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    /// # 从数组中构建完全二叉树
    pub fn from(data: Vec<Option<i32>>) -> Option<Rc<RefCell<Self>>> {
        Self::from_rec(&data, 0)
    }

    #[inline]
    fn from_rec(data: &Vec<Option<i32>>, i: usize) -> Option<Rc<RefCell<Self>>> {
        if i >= data.len() {
            return None;
        }
        if let Some(val) = data[i] {
            let node = Self {
                val,
                left: Self::from_rec(data, 2 * i + 1),
                right: Self::from_rec(data, 2 * i + 2),
            };
            Some(Rc::new(RefCell::new(node)))
        } else {
            None
        }
    }
}
