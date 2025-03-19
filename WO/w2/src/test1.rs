// use yansi and to write some colored text 

use yansi::Paint;
use cfonts::{say, Options};

pub fn col_text1() {
    println!("{} {} {}",
        Paint::red("Hello"),
        Paint::green("PantySmeller"),
        Paint::blue("!")
    );
}

pub fn ban1() {
    say(Options {
        text: String::from("hello"),
        ..Options::default()
    });
}