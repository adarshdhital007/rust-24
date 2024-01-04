// 2870 Minimum Number of Operations to Make Array Empty

use std::collections::HashMap;

pub fn min_operations(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut mp: HashMap<i32, i32> = HashMap::new();

    for i in 0..n {
        *mp.entry(nums[i]).or_insert(0) += 1;
    }

    let mut result = 0;

    for (_, freq) in mp {
        if freq == 1 {
            return -1;
        }

        result += ((freq as f64) / 3.0).ceil() as i32;
    }

    result
}

fn main() {
    let nums = vec![1, 1, 1, 2, 2, 1, 1, 2, 3, 3];
    let result = min_operations(nums);
    println!("Result: {}", result);
}
