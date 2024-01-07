// 446. Arithmetic Slices II - Subsequence
use std::collections::HashMap;

fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
    let size = nums.len();
    let mut dp = vec![HashMap::new(); size];

    let mut total_slices = 0;

    for end in 1..size {
        for start in 0..end {
            let difference = nums[end] as i64 - nums[start] as i64;
            let current_slices = *dp[start].get(&difference).unwrap_or(&0);

            dp[end]
                .entry(difference)
                .and_modify(|count| *count += current_slices + 1)
                .or_insert(current_slices + 1);
            total_slices += current_slices;
        }
    }
    total_slices
}

fn main() {
    let nums = vec![2, 4, 6, 8, 10];
    let result = number_of_arithmetic_slices(nums.clone());

    println!("Input Vector: {:?}", nums);
    println!("Number of Arithmetic Slices: {}", result);
}
