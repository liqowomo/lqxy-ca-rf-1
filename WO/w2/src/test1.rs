// use yansi and to write some colored text
#![allow(dead_code)] // Required to remove dead code warning

use cfonts::{Colors, Fonts, Options, Rgb, say};
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

// Simple banner
pub fn ban1() {
    let string = "Niggaz";

    say(Options {
        text: String::from(string),
        font: Fonts::FontChrome,
        colors: vec![Colors::Red, Colors::Rgb(Rgb::Val(20, 216, 79))],
        ..Options::default()
    });
}
