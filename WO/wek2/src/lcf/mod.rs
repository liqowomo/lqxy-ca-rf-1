// Loope and control flow
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::utils::{header, print_with_synthwave_gradient};
use yansi::Paint;

// Main function that will call other functions in the file
pub fn lcf_main() {
    print_with_synthwave_gradient("Loops + Control Flow".to_string());
    loops_1();
}

// Loops function

fn loops_1() {
    header("Loop Test");
    let mut x = 1;

    // continue loop until x > 5
    loop {
        println!("{}, {}", "x is".blue(), x.magenta());
        x += 1;
        if x > 5 {
            break;
        }
    }
}
