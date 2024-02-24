use std::collections::HashMap;

fn main() {
    let mut elist = vec![1,5,2,6,3,4,123,52,3];
    let mean = calculate_mean(&elist);
    let median = calculate_median(&elist);
    let mode = calculate_mode(&elist);
    println!("mean: {mean}");
    println!("median: {median}");
    println!("mode: {mode}");
}

fn calculate_mean(input_list: &Vec<i32>) -> f64 {
    let mut m = 0;
    for value in input_list {
        m += *value;
    }
    (m/input_list.len() as i32) as f64
}


fn calculate_median(input_list: &Vec<i32>) -> f64 {
    let length_list = input_list.len() as i32;
    let mut sorted_list = input_list.clone();
    sorted_list.sort();
    let mid_idx = sorted_list.len() / 2;

    if length_list%2 == 1 {
        sorted_list[mid_idx] as f64
    } else {
        let sorted_vector = sorted_list[mid_idx-1..mid_idx+1].to_vec();
        calculate_mean(&sorted_vector)
    }
}

fn calculate_mode(input_list: &Vec<i32>) -> i32 {
    let mut occurrences = HashMap::new();

    for &value in input_list {
        let count = occurrences.entry(value).or_insert(0);
        *count += 1;
    }

    occurrences.into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of an empty list")
}