fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    let second: &i32 = &v[1];
    println!("second element using index: {second}");
    let second: Option<&i32> = v.get(1);
    match second {
        Some(elem) => println!("second element using get: {elem}"),
        None => println!("vector doesn't have second element"),
    }

    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 1;
    }
    println!("updated v: {v:#?}");

    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(1),
        SpreadSheetCell::Float(2.3),
        SpreadSheetCell::Text(String::from("test")),
    ];
}

