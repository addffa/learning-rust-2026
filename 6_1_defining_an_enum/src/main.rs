enum IPAddress {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // pass
    }
}

fn main() {
    let home = IPAddress::V4(String::from("127.0.0.1"));

    let loopback = IPAddress::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(7);
    let no_number: Option<i32> = None;
}
