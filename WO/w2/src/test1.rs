// use yansi and to write some colored text 

use yansi::Paint;

pub fn test1() {
    println!("{} {} {}",
        Paint::red("Hello"),
        Paint::green("world"),
        Paint::blue("!")
    );
}