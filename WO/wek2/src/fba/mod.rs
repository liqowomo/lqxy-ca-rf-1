// Loope and control flow
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(deprecated)]

use crate::utils::{clear_console, header, print_with_synthwave_gradient};
use std::io;
use yansi::Paint;

// Main function that will call other functions in the file
pub fn fba_main() {
    // clear_console();
    print_with_synthwave_gradient("Function Basics".to_string());
    split_strings_call();
}

// ******************
// Various number operations
// ******************

// Variou operations on numbers
fn proc_num(numbers: &[i32]) {
    header("Various number operations");

    // Init sum to 0
    let mut sum = 0;

    // Iterate over the numbers, adding each one to sum
    for num in numbers {
        sum += num;
    }

    // Print sum of numbers
    println!("Sum of numbers: {}", sum.bright_blue());

    // If sum even print message
    if sum % 2 == 0 {
        println!("Sum is even");
    } else {
        println!("Sum is odd");
    }
}

fn proc_num_call() {
    // Create a vector of numbers
    let numbers = vec![1, 2, 3, 4, 5];
    let numbers2 = [100, 200, 300, 400, 500];

    // Call proc_num with the vector
    proc_num(&[100, 200, 300, 400, 500]);
    println!("Numbers: {:?}", numbers2.green());
}

// Explicit return tyoes

fn split_strings(s: String, delimiter: char, field: usize) -> String {
    header("Split Strings");
    let parts: Vec<&str> = s.split(delimiter).collect();
    let result = parts.get(field);

    result.expect("Fuck").to_string()
}
fn split_strings_call() {
    let chunk = split_strings("Smell, Panty".to_string(), ',', 1);
    println!("Chunk: {}", chunk.magenta());
}

// Using parameters
fn sum_arg(n: &[i32]) -> i32 {
    let mut result = 0;
    for number in n {
        result += number
    }
    result
}

fn sum_arg_call() {
    let num = [2, 5, 6, 7, 8];
}
