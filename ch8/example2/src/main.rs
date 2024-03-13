use std::collections::HashMap;

fn main() {
    let mut example_list = vec![4,2,16,3,4,5,2,7,71,34,2,3,1,81];
    let mean = calcualte_mean(&example_list);
    let median = calculate_median(&example_list);
    let mode = calculate_mode(&example_list);

    println!("mean: {mean}");
    println!("median: {median}");
    println!("mode: {mode}");
    println!("{:?}", example_list);
}

fn calcualte_mean(input_list: &Vec<i32>) -> f64 {
    let mut total = 0;
    for value in input_list {
        total += *value;
    }
    total as f64/input_list.len() as f64
}

fn calculate_median(input_list: &Vec<i32>) -> f64 {
    let length = input_list.len() as i32;
    let mut input_list_sorted = input_list.clone();
    input_list_sorted.sort();
    let mid_idx = input_list.len()/2; 

    if length%2 == 0 {
        ((input_list_sorted[mid_idx-1] + input_list_sorted[mid_idx]) / 2) as f64
    } else {
        input_list_sorted[mid_idx] as f64
    }
}

fn calculate_mode(input_list: &Vec<i32>) -> i32 {
    let mut hash = HashMap::new();
    for v in input_list {
        *hash.entry(v).or_insert(0) += 1;
    }

    hash.into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| *val)
        .expect("Cannot compute with empty list")
}