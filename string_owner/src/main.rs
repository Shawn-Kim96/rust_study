fn main() {
    let mut s1 = String::from("hello");

    calculate_length(&mut s1);
    {
        let r2 = &mut s1;
        println!("{}", r2);
    }
    println!("{}", s1);

}

fn calculate_length(s: &mut String) {
    s.push_str(", world");
}