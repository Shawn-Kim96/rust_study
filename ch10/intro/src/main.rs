use std::cmp::PartialOrd;

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let n1 = largest(&numbers);
    println!("{}", *n1);

    let characters = vec!['a','b','c'];
    let c1 = largest(&characters);
    println!("{}", *c1);
}
