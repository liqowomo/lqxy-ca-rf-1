// Introduction to Rust Chapter

use crate::utils::print_with_fire_gradient;

pub fn intr_main() {
    bannr();
}

fn bannr() {
    let string = "Introduction to Rust".to_string();
    print_with_fire_gradient(string);
}
