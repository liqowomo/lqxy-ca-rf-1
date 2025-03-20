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
    // Vaible Assigment
    let message = "Panty Smellers";
    let fetish_number = 69;
    let booty_weight = fetish_number / 2;

    println!(
        "{} {} {}",
        message.bold(),
        fetish_number.on_magenta(),
        booty_weight.on_yellow()
    );
}
