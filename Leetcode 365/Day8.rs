// 938. Range Sum of BST
use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq, Clone)]
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

pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    match root {
        Some(node) => {
            let node_ref = node.borrow();
            let node_val = node_ref.val;

            let mut total_sum = if node_val >= low && node_val <= high {
                node_val
            } else {
                0
            };

            // Explore the left subtree if curr > lower bound
            if node_val > low {
                total_sum += range_sum_bst(node_ref.left.clone(), low, high);
            }

            // Explore the right subtree if curr < upper bound
            if node_val < high {
                total_sum += range_sum_bst(node_ref.right.clone(), low, high);
            }

            total_sum
        }
        None => 0,
    }
}

fn main() {
    // BST :
    //        10
    //       /  \
    //      5    15
    //     / \     \
    //    3   7     18

    let root = Some(Rc::new(RefCell::new(TreeNode::new(10))));
    let left = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    let right = Some(Rc::new(RefCell::new(TreeNode::new(15))));
    let left_left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let left_right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
    let right_right = Some(Rc::new(RefCell::new(TreeNode::new(18))));

    root.as_ref().unwrap().borrow_mut().left = left.clone();
    root.as_ref().unwrap().borrow_mut().right = right.clone();
    left.as_ref().unwrap().borrow_mut().left = left_left.clone();
    left.as_ref().unwrap().borrow_mut().right = left_right.clone();
    right.as_ref().unwrap().borrow_mut().right = right_right.clone();

    let result = range_sum_bst(root, 7, 15);
    println!("Sum of values in the range: {}", result);
}
