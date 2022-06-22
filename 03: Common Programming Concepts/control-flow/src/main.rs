fn main() {
    // if Expressions
    println!("IF EXPRESSIONS:");
    let number = 7;
    if number < 5 {
        // this must be a bool
        println!("less than 5");
    } else if number < 3 {
        println!("less than 3");
    } else {
        println!("not less than 3 or 5");
    }
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("condition number is {}", number);

    // loop
    println!("\nLOOPS:");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                // break a loop with a label
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    // return values from loops:
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The value of result is {}", result);

    // while loops:
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!");

    // for loops:
    let array = [10, 20, 30, 40, 50];
    let mut i = 0;
    while i < 5 {
        println!("the value is {}", array[i]);
        i += 1;
    }
    // is the same as:
    for element in array {
        println!("the value is {}", element);
    }

    // iterate through a range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");
}
