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
    ownership_ring();
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

#[inline(never)] // This instruction forces the function to be seperately compiled
fn ownership_ring() {
    let saurons_ring = MyPreciousRing::forge();
    println!("Sauron Ring Sez : {saurons_ring:#?}");
    println!("Sauron Ring Sez : {saurons_ring:#?}");

    let gollums_ring = saurons_ring;
    println!("My Precious : {gollums_ring:#?}");
}
