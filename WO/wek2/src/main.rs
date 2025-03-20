// Main Entry Point

mod INTR;
mod test;
mod utils; // required to have it called in other parts of project 
use INTR::intr_main;

fn main() {
    intr_main();
}
