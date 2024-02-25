use std::fmt::Display;

fn longest_with_an_announcement<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}



// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

fn main() {
    // let novel = String::from("Call me sdfqwer. I'm home");
    // let first_sentence = novel.split('.')
    //     .next()
    //     .expect("Could not find a '.'");
    // let i = ImportantExcerpt { part: first_sentence };
    println!("A");
}