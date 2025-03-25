//Work to study ownership and borrowing model
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

// --- Imports Section ---
use crate::utils::{clear_console, header, print_with_synthwave_gradient};
use yansi::Paint;

// --- Functions Section --

// Main Function entry point

pub fn w2_main() {
    // Banner Section
    let banner = "Ownership Model ";
    print_with_synthwave_gradient(banner.to_string());
    println!("{}", "Rust Ownership Model".green());

    // Calling Sub Functions
}

// --- Sub functions to be called in main ---

#[derive(Debug)]
pub struct MyPreciousRing {
    pub engraving: String,
}

impl MyPreciousRing {
    pub fn forge() -> Self {
        Self {
            engraving: "The One Ring".to_string(),
        }
    }
}
