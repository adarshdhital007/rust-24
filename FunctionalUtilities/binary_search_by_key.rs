fn main() {
    let data = vec![
        (1, "Hemanta Dai", "Software Engineer"),
        (2, "Adarsh", "Fullstack Developer"),
        (3, "Chris", "Frontend Developer"),
        (4, "Prakash", "Frontend Developer")
    ];

    let key_to_search = 2;
    let result = data.binary_search_by_key(&key_to_search, |&(key, _, _)| key);

    match result {
        Ok(index) => {
            let (_, name, role) = data[index];
            println!("Found at index {}: {} - {}", index, name, role);
        }
        Err(_) => println!("Key not found"),
    }
}
