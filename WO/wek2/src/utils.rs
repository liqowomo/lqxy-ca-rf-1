// main utiities file

use cfonts::{Fonts, Options, say};

pub fn print_with_fire_gradient(text: String) {
    // Define the fire gradient colors
    let synth = vec![
        String::from("#FF00FF"), // Neon Pink
        String::from("#8A2BE2"), // Blue Violet
        String::from("#00FFFF"), // Cyan
        String::from("#FF1493"), // Deep Pink
        String::from("#9400D3"), // Dark Violet
    ];

    say(Options {
        text,
        font: Fonts::FontTiny,
        gradient: synth,
        independent_gradient: false,
        transition_gradient: true,
        ..Options::default()
    });
}
