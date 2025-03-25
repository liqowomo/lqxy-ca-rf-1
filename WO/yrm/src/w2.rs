//Work to study ownership and borrowing model
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

// --- Imports Section ---

use crate::utils::{clear_console, header, print_with_synthwave_gradient};
use yansi::Paint;

// --- Functions Section --

pub fn w2_main() {
    // Banner Section
    let banner = "Ownership Model ";
    print_with_synthwave_gradient(banner.to_string());
    println!("{}", "Rust Ownership Model".green());
}
