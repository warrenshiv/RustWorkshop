use std::io;

fn task_1() {
    for i in (1..=50).rev() {
        println!("{}", i);
    }
}

fn task_2() {
    println!("Enter a number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num: i32 = input.trim().parse().expect("Please enter a valid number");

    for i in 0..=num {
        println!("{}", i);
    }
}

fn task_3() {
    println!("Enter first number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num1: f64 = input.trim().parse().expect("Please enter a valid number");

    println!("Enter second number:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num2: f64 = input.trim().parse().expect("Please enter a valid number");

    println!("Addition: {}", num1 + num2);
    println!("Subtraction: {}", num1 - num2);
    println!("Multiplication: {}", num1 * num2);
    println!("Division: {}", num1 / num2);
}

fn main() {
    loop {
        println!("Select a task to run:");
        println!("1. Print numbers in reverse from 50 to 1");
        println!("2. Print numbers from 0 to a given input");
        println!("3. Perform basic arithmetic operations");
        println!("4. Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let choice: i32 = input.trim().parse().expect("Please enter a valid number");

        match choice {
            1 => task_1(),
            2 => task_2(),
            3 => task_3(),
            4 => break,
            _ => println!("Invalid choice. Please enter a valid number."),
        }
    }
}
