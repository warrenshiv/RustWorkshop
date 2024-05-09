fn main() {
    // Integer data types
    let a: i8 = 10;
    let b: i16 = 20;
    let c: i32 = 30;
    let d: i64 = 40;
    let e: i128 = 50;
    let f: isize = 60;

    // Print Integer data types
    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);
    println!("e: {}", e);
    println!("f: {}", f);

    // Floating point data types
    let g: f32 = 10.0;
    let h: f64 = 20.0;

    // Print Floating point data types
    println!("g: {}", g);
    println!("h: {}", h);

    // Character data type
    let i: char = 'A';

    // Print Character data type
    println!("i: {}", i);

    // String data type
    // str = taken as a constant.
    // String = a heap-allocated string


    let j: &str = "Rust Programming Language";
    let k: String = String::from("Hello, Rust!");

    // Print String data type
    println!("j: {}", j);
    println!("k: {}", k);
}
