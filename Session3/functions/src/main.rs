fn main() {
    sum1(10, 20);

    sum2(30, 40);
}

// Function without return type
fn sum1(num1: i32, num2: i32) {
    let num3 = num1 + num2;
    println!("The sum of {} and {} is {}", num1, num2, num3);
}


// Function with return type(Explicit return type)
fn sum2(num1: i32, num2: i32) -> i32 {
    let num3 = num1 + num2;
    println!("The sum of {} and {} is {}", num1, num2, num3);
    return num3;
}

// 