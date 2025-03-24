// Work 1 from the tutorial
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

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
    play2();
}

// ********* Test Functions ***************

// Testing multine text printing
fn testrun() {
    let line = "+".repeat(20);
    println!(
        "
{}
{}
{}f
",
        line, "Hey", "What"
    );
}

// ********* Test Functions ***************
#[inline(never)]
fn play1() {
    header("Playground");

    // Variables by defalt are immutable
    let mut x = 42;
    x += 1;
    println!("{}", x);
}

#[inline(never)]
fn play2() {
    println!("{}", "Play 2 Function".green());

    let numbers = [0x68, 0x69, 0x0a, 0];
    let numbers2 = vec![0x68, 0x69, 0x0a, 0];
    println!("{:?}", numbers);
}
