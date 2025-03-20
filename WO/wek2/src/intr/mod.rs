// Introduction to Rust Chapter

use crate::utils::print_with_fire_gradient;
use yansi::Paint;

pub fn intr_main() {
    bannr();
    demo_variable_assignment();
    ctrl_flow_1();
}

fn bannr(text: &str) {
    let string = text.to_string();
    print_with_fire_gradient(string);
}

fn demo_variable_assignment() {
    // Vaible Assigment
    let message = "Panty Smellers";
    let fetish_number = 69.2;
    let booty_weight = fetish_number / 2.2;
    let line = "~".repeat(20);

    println!(
        "Demo Variables \n{} \n {} {} {}",
        line.blue(),
        message.bold(),
        fetish_number.magenta(),
        booty_weight.yellow(),
    );
}

// learning about control flow here
fn ctrl_flow_1() {
    let proceed = true;
    if proceed {
        println!("{}", "Sniffing".green());
    } else {
        println!("Not Proceeding");
    }
}
