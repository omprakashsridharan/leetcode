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
}

use std::cell::RefCell;
use std::rc::Rc;

pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let n = nums.len();
    if n == 0 {
        return None;
    }
    let mid = n / 2;
    let mut root = TreeNode::new(*nums.get(mid).unwrap());
    root.left = sorted_array_to_bst(nums[0..mid].to_vec());
    root.right = sorted_array_to_bst(nums[mid + 1..n].to_vec());
    return Some(Rc::new(RefCell::new(root)));
}
