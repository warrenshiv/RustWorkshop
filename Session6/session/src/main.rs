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
    } else {
        println!("{a} is greater than {b}")
    }

    // Loops
    // let network_response :i32 = 200;
    // let is_success :bool = network_response

    let mut counter: i32 = 0;
    loop {
        counter += 1;
        println!("Counter: {}", counter);

        if counter == 1 {
            println!("Starting point");
            continue;
        }

        if counter == 10 {
            break;
        }
    }

    // while loop
    let mut number: i32 = 5;

    while number != 0 {
        println!("{number}!");

        number -= 1;

        println!("While loop");
    }

    

    let array: [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

    let mut index: usize = 5;
    while index > 0 {
        index -= 1;
        println!("We are looping on item {}", array[index]);

    }
    

    // for loop
    let array: [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

    for element in array{
        println!("We are looping in item {element}");
    }
}
