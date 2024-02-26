use std::io;

// // C -> F or F -> C

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


// fn f_to_c(f: f64) -> f64 {
//     (f - 32.0)*5.0/9.0
// }

// fn c_to_f(c: f64) -> f64 {
//     (9.0*c/5.0) + 32.0
// }


// n번째 피보나치 수열 생성.
// 

fn main() {
    let mut n = String::new();
    println!("Enter the number of n in integer");

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read n");
    
    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter valid number");
            return;
        },
    };

    let fibonnacci = fib(n);

    println!("{fibonnacci}");
}

fn fib(n: u32) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut a: u64 = 0;
    let mut b: u64 = 1;
    let mut temp: u64 = 0;

    for _ in 2..=n {
        temp = a + b;
        a = b;
        b = temp;
    }
    return temp;
}