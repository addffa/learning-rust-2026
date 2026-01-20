#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // we can have a method that has same name with struct field
    fn width(&self) -> bool {
        self.width > 0
    }

    // example of more params
    fn can_hold(&self, other_rec: &Self) -> bool {
        self.width >= other_rec.width && self.height >= other_rec.height
    }

    // associated function of the structs
    fn square(size: u32) -> Self {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// it possible to have separated impl block with same structs
impl Rectangle {
    fn perimeter(&self) -> u32 {
        (self.width + self.height) * 2
    }
}

fn main() {
    let rec = Rectangle {
        width: 30,
        height: 50,
    };
    println!("area of rec {:#?} is {}", rec, rec.area());

    if rec.width() {
        println!("the rectangle has width");
    }

    let area = (&rec).area(); // is the same width rec.area();
    println!("area: {area}");

    let other_rec = Rectangle {
        width: 20,
        height: 30,
    };
    println!("rec can hold other_rec: {}", rec.can_hold(&other_rec));

    let long_rec = Rectangle {
        width: 100,
        height: 10,
    };
    println!("rec can hold long_rec: {}", rec.can_hold(&long_rec));

    let square = Rectangle::square(30);
    println!("this square {:#?} has area {}", square, square.area());

    println!("the rec perimeter: {}", rec.perimeter());
}

