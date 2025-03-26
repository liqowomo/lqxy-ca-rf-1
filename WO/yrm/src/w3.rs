//Work to study ownership and borrowing model
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

// --- Imports Section ---
use crate::utils::{clear_console, header, print_with_synthwave_gradient};
use yansi::Paint;

// Main Function call
pub fn w3_main() {
    // Banner Section
    let banner = "Ownership Model ";
    print_with_synthwave_gradient(banner.to_string());
    header("3rd Version of the file");

    // Calling Sub Functions
}

// Sub Functions
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

#[inline(never)] // This instruction forces the function to be seperately compiled
fn happy_lor() {}
