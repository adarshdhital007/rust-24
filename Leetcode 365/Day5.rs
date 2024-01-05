// 300. Longest Increasing Subsequence
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    if nums.is_empty(){
        return 0;
    }

    let mut lis = Vec::new();

    for &num in &nums{
        if let Err(idx) = lis.binary_search(&num){
            if idx == lis.len(){
                lis.push(num);
            }
            else
            {
                lis[idx] = num;
            }
        }
    }
    lis.len() as i32
}

fn main(){
    let nums = vec![100,98,97,2023,2024];
    let result = length_of_lis(nums.clone());
    println!("Length of the Longest Increasing Subsequence: {}", result);
}
