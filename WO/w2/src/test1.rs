// use yansi and to write some colored text
#![allow(dead_code)] // Required to remove dead code warning
#![allow(unused_imports)] // Allowin unused imports

use cfonts::{BgColors, Colors, Fonts, Options, Rgb, say};
use sha256::{Digest, Sha256};
use yansi::Paint;

// Simple colored text with yansi
pub fn col_text1() {
    println!(
        "{} {} {}",
        Paint::red("Hello"),
        Paint::green("PantySmeller"),
        Paint::blue("!")
    );
}

// Simple banner - Note using cfonts
pub fn ban1() {
    let string = "BootyDance";

    say(Options {
        text: String::from(string),
        font: Fonts::FontChrome,
        gradient: vec![String::from("#ff8800"), String::from("#88ff00")],
        ..Options::default()
    });
}

// Function for dinding the SHA256 of a number
pub fn sha256(num: u32) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(num.to_string());
    let result = hasher.finalize();
    format!("{:x}", result)
}
