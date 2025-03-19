// use yansi and to write some colored text

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
    say(Options {
        text: String::from("hello"),
        ..Options::default()
    });

    let string = "PantySmeller";

    say(Options {
        text: String::from(string),
        font: Fonts::FontSlick,
        colors: vec![Colors::Red, Colors::Rgb(Rgb::Val(20, 216, 79))],
        ..Options::default()
    });
}
