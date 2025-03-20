// main utiities file

use cfonts::{Fonts, Options, say};

pub fn print_with_fire_gradient(text: String) {
    // Define the fire gradient colors
    let fire_gradient = vec![
        String::from("#FF4500"), // Firebrick red
        String::from("#FFFF00"), // Yellow
    ];

    say(Options {
        text,
        font: Fonts::FontPallet,
        gradient: fire_gradient,
        independent_gradient: false,
        transition_gradient: true,
        ..Options::default()
    });
}
