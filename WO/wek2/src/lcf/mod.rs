// Loope and control flow
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::utils::{header, print_with_synthwave_gradient};
use std::io;
use yansi::Paint;

// Main function that will call other functions in the file
pub fn lcf_main() {
    print_with_synthwave_gradient("Loops + Control Flow".to_string());
    if_let_1();
}

// Loops function

fn loops_1() {
    header("Loop Test");
    let mut x = 1;
    // continue loop until x > 5
    loop {
        println!("{}, {}", "x Looped to".blue(), x.magenta());
        x += 1;
        if x > 5 {
            break;
        }
    }
}

// Enahnced Loops function that will accept input and then loop
fn loops_2() {
    header("Enhanced Loop Test");

    let mut input = String::new();
    println!("➡️ Fucker Put Number: ");
    io::stdin().read_line(&mut input).unwrap();

    // Handle invalid or empty input
    let input: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "Invalid input. Please enter a valid number.".red());
            return;
        }
    };

    let mut x = 1; // Start the loop counter
    loop {
        println!("{}, {}", "x Looped to".blue(), x.magenta());
        x += 1;
        if x > input {
            break;
        }
    }
}

// Exploring if let
fn if_let_1() {
    header("If Let Test");
    // let mut maybe_number = some(69)
    let may_num = Some(69);
    if let Some(num) = may_num {
        println!("Number: {}", num.magenta());
    } else {
        println!("No number");
    }
}
