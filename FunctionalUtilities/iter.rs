// fn main() {
//     let mut numbers = vec![1, 2, 3, 4, 5];

//     let mut iter = numbers.iter();

//     // Try to modify an element through the iterator
//     if let Some(&number) = iter.next() {
//         // This line would result in a compilation error
//         // because `number` is an immutable reference.
//         number += 1;
//         println!("Modified element: {}", number);
//     }
// }

// If we want to create an mutable reference then use iter_mut()

fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];

    // Using `iter_mut()` to create a mutable iterator
    for number in numbers.iter_mut() {
        // We are modifying the number 
        *number *= 2; 
    }

    println!("Modified vector: {:?}", numbers);
}
