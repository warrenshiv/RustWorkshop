fn main() {

    // Ownership

    // Borrow and Move

    // Ownership Rules
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    // Stack(Cheap/Fast) -> int, str, float, bool, char, tuple, array
    // Heap(Large/slow) -> Strings, Vectors, hashmaps

    // Annotated with ownership
    // Stack
    let a : i32 = 10;
    let b : i32 = a;
    println!("b: {b}");
    


    // Heap
    let mut t1 :String = "Rust".to_string();

    t1.push_str(" Programming");
    println!("t: {t1}");
    


    let c :String = String::from("Hello");
    let d =  example(c);
    println!("d: {d}");

    // Borrow Example
    let e :String = String::from("Borrow Example");
    let f = borrow_example(&e);
    println!("f: {f}");



    let mut g :String = String::from("Hello");
    change(&mut g);
    println!("g: {g}");

    
}


// fn hello(param_1: String) -> String {
//     println!("param_1: {param_1}");
//     param_1
// }

fn example(s: String) -> String {
    println!("s: {s}");
    s
}


fn borrow_example(par: &String) -> String {
    par.to_string()
}


fn change(some_string: &mut String) {
    some_string.push_str(", world");
}


// Dangling pointer error - 
// fn dangle() -> &str {
//     return "hi"
// }




// Read through UnderStanding Ownership in Rust
