enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call (&self) {
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();


    let some_numver = Some(5);
    let some_string = Some("a string");
    let some_number = Some(6);

    // println!("{}", some_number + some_numver);
}

enum Option<T> {
    Some(T),
    None,
}