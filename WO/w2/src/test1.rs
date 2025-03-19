// use yansi and to write some colored text 

use yansi::Paint;

pub fn coltext1() {
    println!("{} {} {}",
        Paint::red("Hello"),
        Paint::green("PantySmeller"),
        Paint::blue("!")
    );
}