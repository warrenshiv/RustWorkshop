fn main() {
    // Conditions

    // greater than
    // equal to
    // less

    let a: i32 = 50;
    let b: i32 = 20;

    if a > b {
        println!("{a} is greater than {b}")
    }

    if a == b {
        println!("{a} is equal to {b}")
    }

    if a < b {
        println!("{a} is less than {b}")
    } else if a == b {
        println!("{a} is equal to {b}")
    }else{
        println!("{a} is greater than {b}")
    }



    // Loops
    let mut counter : i32 = 0;
    loop {
        counter += 1;
        println!("Counter: {}", counter);
        if counter == 10 {
            break;
        }
    }
}
