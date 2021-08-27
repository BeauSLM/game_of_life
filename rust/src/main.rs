extern crate rand;
use std::{thread, time};

fn census(world: [[bool; 75]; 75]) -> u16 { // accepts a double array slice of unknown length -- maybe find a way to enforce a square/rectangular grid in the function signature?
    let mut count = 0;

    for i in 0..75 {
        for j in 0..75 {
            if world[i][j] {
                count += 1;
            }
        }
    }
    count
}

// advances the game according to the rules, generating and deleting life forms as needed
// fn generation(world: [[bool; 75]; 75) -> [[bool; 75]; 75] {
//     let mut new_world: [[bool; 75]; 75];
// }

fn main() {
}
