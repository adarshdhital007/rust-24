fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let nlist = [1,2,3,4,5,6,7];
    let result = largest_i32(&nlist);
    println!("The largest element is: {}", result);

    let clist = ['a', 'b', 'c', 'd', 'e'];
    let result = largest_char(&clist);
    println!("The largest element is: {}", result);
}
