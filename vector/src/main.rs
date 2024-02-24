#[derive(Debug)]
enum SpreadsheelCell {
    Int(i32),
    Float(f64),
    Text(String),
}


fn main() {
    let row = vec![
        SpreadsheelCell::Int(3),
        SpreadsheelCell::Float(6.4),
        SpreadsheelCell::Text(String::from("hi")),
    ];

    println!("{:?}", row);
}