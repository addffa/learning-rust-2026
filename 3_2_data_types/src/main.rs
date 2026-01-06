fn main() {
    // numeric type
    // addition
    let sum = 5 + 10;
    println!("{sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("{difference}");

    // multiplication
    let product = 4 * 30;
    println!("{product}");

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("{quotient} {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("{remainder}");
    
    // boolean type
    let t = true;

    let f: bool = false; // with explicit type annotation
    println!("{t} {f}");
    
    // character type
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{c} {z} {heart_eyed_cat}");

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The values: {x}, {y}, {z}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("acessing using period: {five_hundred}, {six_point_four}, {one}");

    // array type
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // generate [3, 3, 3, 3, 3]
    let first = a[0];
    let second = a[1];
    println!("{first}, {second}");
}
