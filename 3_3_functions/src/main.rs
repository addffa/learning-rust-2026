fn main() {
    println!("Hello, functions!");

    another_function();

    another_function_with_param(7);

    print_labeled_measurement(9, 'x');

    let x = {
        let y = 3;
        y + 2
    };
    println!("The value x: {x}");

    let res = plus_twenty_one(2);
    println!("{res}");
}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_param(x: i32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("Measurement is: {value}{unit_label}");
}

fn plus_twenty_one(x: i32) -> i32 {
    x + 21
}
