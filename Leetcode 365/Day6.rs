// 1235 Maximum Profit in Job Scheduling
fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
    let mut jobs: Vec<usize> = (0..profit.len()).collect();
    jobs.sort_unstable_by_key(|&i| end_time[i]);

    let mut v: Vec<(i32, i32)> = vec![(0, i32::MIN)];

    for &i in jobs.iter() {
        let left = match v.binary_search_by_key(&start_time[i], |&(_, t)| t) {
            Ok(j) => j,
            Err(j) => j - 1,
        };

        if v[left].0 + profit[i] > v.last().unwrap().0 {
            if end_time[i] == v.last().unwrap().1 {
                v.last_mut().unwrap().0 = v[left].0 + profit[i];
            } else {
                v.push((v[left].0 + profit[i], end_time[i]));
            }
        }
    }
    v.last().unwrap().0
}

fn main() {
    let start_time = vec![1, 2, 3, 3];
    let end_time = vec![3, 4, 5, 6];
    let profit = vec![50, 10, 40, 70];

    let result = job_scheduling(start_time, end_time, profit);

    println!("Maximum Profit: {}", result);
}