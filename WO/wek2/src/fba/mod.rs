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
}

// Variou operations on numbers
fn proc_num(numbers: &[i32]) {
    // Init sum to 0
    let mut sum = 0;

    // Iterate over the numbers, adding each one to sum
    for num in numbers {
        sum += num;
    }

    // Print sum of numbers
    println!("Sum of numbers: {}", sum.on_bright_blue());
}
