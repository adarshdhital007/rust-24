fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let even_numbers: Vec<_> = numbers.into_iter().filter(|&x| x % 2 == 0).collect();
    
    println!("Filtered numbers : {:?}",even_numbers);
}
