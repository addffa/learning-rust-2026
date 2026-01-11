fn main() {
    // variable scope
    {
        let s = "hello";
        println!("variable s = {s} in this scope");
    } // end of variable s
    
    // `String` type for example of reference type that use heap to allocate data
    let mut s = String::from("hello");
    s.push_str(", world!"); // push literal string into `String`
    println!("{s}");

    // rust automatically drop to return memory on the heap when the variable out of scope
    {
        let s = String::from("hello from `String` in scope");
        println!("{s}");
    } // out of scope, s is no longer valid

    // variables and data interacting with move
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved here
    // println!("{s1}"); will not compile
    println!("the pointer s2 has value: {s2}");

    // variables and data interacting with clone
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("value from s1: {s1}");
    println!("value from s2: {s2}");

    // stack-only data: copy
    let x = 5;
    let y = x;
    println!("x = {x} = y = {y}");

    // ownership and functions
    let s = String::from("hello");
    takes_ownership(s); // s moves into the function, no longer valid after this
    {
        let x = 5;
        makes_copy(x); // x doesn't move into the function, because i32 implement Copy trait
        // ... x stil valid in here
    } // x out of scope
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

