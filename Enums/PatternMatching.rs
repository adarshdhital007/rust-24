fn main() {
    let goat = 7;
    let number2 = 10;
    let reference = &Some(7);

    match goat {
        0 => println!("zero"),
        1 | 2 => println!("one or two"),
        7 => println!("Ronaldo"),
        3..=9 => println!("Some other number"),
        _ => println!("Not  a number"),
    }

    match number2 {
        _ => println!("Not  a goat"),
    }

    match reference {
        &Some(value) => println!("Value: {}", value),
        &None => println!("None"),
    }

    //matching patterns with Guards
    let some_value = Some(7);

    match some_value {
        Some(value) if value > 0 => println!("Postive Value: {}", value),
        Some(value) if value < 0 => println!("Negative Value: {}", value),
        Some(_) | None => println!("Non positive value"),
    }
}
