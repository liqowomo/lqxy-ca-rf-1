// Work 1 from the tutorial
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::utils::{clear_console, header, print_with_synthwave_gradient};
use yansi::Paint;

// Main Function entry point

pub fn w1_main() {
    let banner = "Work 1 ";
    print_with_synthwave_gradient(banner.to_string());
}
