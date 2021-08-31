// TODO: have rust call a python script/function that accepts a size and a name, so that python can
// generate a txt for rust to use

// TODO: create a function that writes the game grid to a file (preferably in coordinate pairs
// specifying live cells)

extern crate rand;
extern crate termion;
use std::process::exit;
use std::{env, thread, time};
use std::fs::File;
use std::io::{BufRead, BufReader};
use termion::{clear, color};

const SIZE: usize = 75;

fn main() {
    let mut world = [[false; SIZE]; SIZE];
    let args: Vec<String> = env::args().collect();

    let mut generations: u8 = 0;
    let max_generations: u8 = get_max_gens(&args);

    if args.len() < 2 {
        for i in 0..SIZE {
            for j in 0..SIZE {
                if rand::random() {
                    world[i][j] = true;
                } else {
                    world[i][j] = false;
                }
            }
        }
    } else {
        let filename = env::args().nth(1).unwrap();
        world = populate_from_file(filename);
    }

    println!("Poulation at generation {} is {}", generations, census(world));
    while generations < max_generations {
        let temp = generation(world);
        world = temp;
        generations += 1;
        println!("{}", clear::All);
        print_world(world);
        println!("{blue}Population at generation {g} is {c}", blue = color::Fg(color::Blue), g = generations, c = census(world));
        thread::sleep(time::Duration::from_secs(2));
    }
}

fn get_max_gens(args: &Vec<String>) -> u8 {
    if args.len() > 2 {
        env::args().nth(2).unwrap().parse::<u8>().unwrap()
    } else {
        100
    }
}

fn census(world: [[bool; SIZE]; SIZE]) -> u16 {
    let mut count = 0;

    for i in 0..SIZE {
        for j in 0..SIZE {
            if world[i][j] {
                count += 1;
            }
        }
    }
    count
}

// advances the game according to the rules, generating and deleting life forms as needed
fn generation(world: [[bool; SIZE]; SIZE]) -> [[bool; SIZE]; SIZE] {
    let mut new_world = [[false; SIZE]; SIZE];
    
    for i in 0..SIZE {
        for j in 0..SIZE {
            let mut count = 0;

            if i > 0 && world[i - 1][j] {
                count += 1;
            }

            if i > 0 && j > 0 && world[i - 1][j - 1] {
                count += 1;
            }

            if i > 0 && j < SIZE - 1 && world[i - 1][j + 1] {
                count += 1;
            }

            if i < SIZE - 1 && j > 0 && world[i + 1][j - 1] {
                count += 1;
            }

            if i < SIZE - 1 && world[i + 1][j] {
                count += 1;
            }

            if i < SIZE - 1 && j < SIZE - 1 && world[i + 1][j + 1] {
                count += 1;
            }

            if j > 0 && world[i][j - 1] {
                count += 1;
            }

            if j < SIZE - 1 && world[i][j + 1] {
                count += 1;
            }

            if count == 3 {
                new_world[i][j] = true;
            } else if count == 2 && world[i][j] {
                new_world[i][j] = true;
            }
        }
    }
    new_world
}

fn print_world(world: [[bool; SIZE]; SIZE]) {
    for i in 0..SIZE {
        for j in 0..SIZE {
            if world[i][j] {
                print!("{red}*", red = color::Fg(color::Red));
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn populate_from_file(filename: String) -> [[bool; SIZE]; SIZE] {
    let mut new_world = [[false; SIZE]; SIZE];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut words = line.split_whitespace();

        let left = words.next().unwrap();
        let left = left.parse::<usize>().unwrap();

        let right = words.next().unwrap();
        let right = right.parse::<usize>().unwrap();

        if left > SIZE - 1 || right > SIZE - 1 {
            eprintln!("File countains value out of valid range - please correct the file or use another.");
            exit(1);
        }

        new_world[left][right] = true;
    }
    
    new_world
}

