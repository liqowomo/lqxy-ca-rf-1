// use yansi and to write some colored text 

use yansi::Paint;
use cfonts::{say, font};

pub fn col_text1() {
    println!("{} {} {}",
        Paint::red("Hello"),
        Paint::green("PantySmeller"),
        Paint::blue("!")
    );
}

pub fn ban1() {
    
}