fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    if number % 5 == 0 {
        println!("number is divisible by 5");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 2 or 3 or 5");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("the value of number is {number}");

    // repetition with loop
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 2 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");

    // loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count: {count}");
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("end count: {count}");

    // while loop
    let  mut number = 3;
    while number > 0 {
        println!("number: {number}");
        number -= 1;
    }
    println!("end while loop: {number}");

    // for loop
    let a = [1, 2, 3, 4, 5];
    for element in a {
        println!("element: {element}");
    }
}
