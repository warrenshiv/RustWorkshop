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
    task_1();
    task_2();
    task_3();
}
