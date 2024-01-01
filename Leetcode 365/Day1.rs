pub fn assign_cookies(greedy_levels: Vec<i32>, cookie_sizes: Vec<i32>) -> i32 {
    let mut greedy_levels = greedy_levels;
    let mut cookie_sizes = cookie_sizes;

    greedy_levels.sort();
    cookie_sizes.sort();

    let mut satisfied_children = 0;
    let mut cookie_index = 0;

    while let (Some(cookie_size), Some(greedy_level)) = (
        cookie_sizes.get(cookie_index),
        greedy_levels.get(satisfied_children),
    ) {
        if cookie_size >= greedy_level {
            satisfied_children += 1;
        }
        cookie_index += 1;
    }

    satisfied_children as i32
}

fn main() {
    let greedy_levels = vec![2, 1, 6, 8];
    let cookie_sizes = vec![13, 3, 3, 1];

    let result = assign_cookies(greedy_levels, cookie_sizes);

    println!("Maximum content children: {}", result);
}
