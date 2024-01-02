//2610 Convert an Array Into a 2D Array With Conditions

use std::collections::HashMap;

pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let n = nums.len();
    let mut mp: HashMap<i32, usize> = HashMap::new();
    let mut result: Vec<Vec<i32>> = Vec::new();

    for i in 0..n {
        let freq = mp.get(&nums[i]).cloned().unwrap_or(0);
        if freq == result.len() {
            result.push(Vec::new());
        }
        result[freq].push(nums[i]);
        *mp.entry(nums[i]).or_insert(0) += 1;
    }
    result
}

fn main() {
    let nums = vec![1,2,3,7];
    let result = find_matrix(nums);
    println!("{:?}", result);
}
