// Work 1 from the tutorial
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::utils::{clear_console, header, print_with_synthwave_gradient};
use yansi::Paint;

// Main Function entry point

// ********* Banner Funcion ***************

pub fn w1_main() {
    // Banner Section
    let banner = "Work 1 ";
    print_with_synthwave_gradient(banner.to_string());
    println!("{}", "Memory Management In Rust".green());

    // Functions Section
    // testrun();
    play1();
}

// ********* Test Functions ***************

// Testing multine text printing
fn testrun() {
    let line = "+".repeat(20);
    println!(
        "
{}
{}
{}
",
        line, "Hey", "What"
    );
}

// ********* Test Functions ***************
#[inline(never)]
fn play1() {
    header("Playground");
}
