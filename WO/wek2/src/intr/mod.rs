// Introduction to Rust Chapter

use crate::utils::print_with_fire_gradient;
use yansi::Paint;

pub fn intr_main() {
    bannr();
    demo_variable_assignment();
}

fn bannr() {
    let string = "Introduction to Rust".to_string();
    print_with_fire_gradient(string);
}

fn demo_variable_assignment() {
    let message = "Panty Smellers";
    let fetish_numer = 69;
    let booty_weight = fetish_numer / 2;

    println!("{} {} {}", message, fetish_numer, booty_weight);
    println!(
        "{} {} {}",
        Paint::red(message),
        Paint::green(fetish_numer),
        Paint::blue(booty_weight)
    );
}
