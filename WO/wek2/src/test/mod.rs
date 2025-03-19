// Testing printing here

use yansi::Paint;

pub fn tpr1() {
    println!("{}", Paint::blue("Hello, from src/test/mod.rs - tpr1"));
}

pub fn tpr2() {
    println!("{}", Paint::yellow("Hello, from src/test/mod.rs - tpr1"));
}
