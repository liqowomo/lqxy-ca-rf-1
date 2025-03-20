// Introduction to Rust Chapter

use crate::utils::print_with_fire_gradient;
use yansi::Paint;

pub fn intr_main() {
    bannr();
    demo_variable_assignment();
    ctrl_flow_1();
}

fn bannr() {
    let string = "Introduction to Rust".to_string();
    print_with_fire_gradient(string);
}

fn header(text: &str) {
    let line = "~".repeat(20);
    println!("{} \n {} \n{}", line.blue(), text.blue(), line.blue());
}

fn demo_variable_assignment() {
    // Vaible Assigment
    let message = "Panty Smellers";
    let fetish_number = 69.2;
    let booty_weight = fetish_number / 2.2;

    header("Variable Assignment");

    println!(
        "{} {} {}",
        message.bold(),
        fetish_number.magenta(),
        booty_weight.yellow(),
    );
}

// learning about control flow here
fn ctrl_flow_1() {
    header("Control Flow");
    let proceed = true;
    if proceed {
        println!("{}", "Sniffing".green());
    } else {
        println!("Not Proceeding");
    }

    // If Else blocks
    let height = 60;
    if height < 69 {
        println!("{}", "Short 69".green());
    } else if height == 69 {
        println!("{}", "Straight 69".blue());
    } else {
        println!("{}", "More 69".red());
    }
}

// mutability 

fn mutability_1() {
    let mut height = 10;
    height = height * 20;

    let result = if height > 200 {
        print!ln("{}")
    }

}