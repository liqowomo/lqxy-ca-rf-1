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

    let gollums_ring = saurons_ring;
    println!("My Precious : {gollums_ring:#?}");

    let bilbos_ring = gollums_ring;
    // println!("Bilbo's Ring Sez : {bilbos_ring:#?}");

    let frodo_ring = bilbos_ring;
    println!("Frodo's Ring Sez : {frodo_ring:#?}");

    let samwise_ring = &frodo_ring;
    println!("Samwise's Ring Sez : {samwise_ring:#?}");
    println!("Frodo's Ring Sez : {frodo_ring:#?}");

    // Shadowing here
    let mut frodos_ring = frodo_ring;
    heat(&mut frodos_ring);
    println!("Frodo's Ring Sez : {frodos_ring:#?}");
}

// Helper function
// Deo of a mutable borrow = &mut
pub fn heat(ring: &mut MyPreciousRing) {
    ring.engraving = "HeatingRing".to_string();
}
