use std::io;

// C -> F or F -> C

fn main() {
    let mut input = String::new();
    let mut temp_type = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("input should be in integer form");
            panic!("Invalid input, program will terminate.");
        },
    };
    
    io::stdin()
        .read_line(&mut temp_type)
        .expect("Failed to get temp_type");

    temp_type = temp_type.trim().to_string();

    if temp_type == "cel" {
        let output = c_to_f(input);
        println!("{input} F` -> {output} C`");
    } else if temp_type == "fer" {
        let output = f_to_c(input);
        println!("{input} C` -> {output} F`");
    } else {
        println!("temp_type should be 'cel' or 'fer'");
        panic!("Invalid input, program will terminate.");
    }
    // println!("{input}");
    // println!("{temp_type}")
}


fn f_to_c(f: f64) -> f64 {
    (f - 32.0)*5.0/9.0
}

fn c_to_f(c: f64) -> f64 {
    (9.0*c/5.0) + 32.0
}