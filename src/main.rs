#[macro_use] extern crate lazy_static;
extern crate regex;

use std::time::{SystemTime, Instant};
use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::io::{BufReader};
use regex::Regex;
use std::char;

fn time_since_start(start: Instant) {
    let elapsed = start.elapsed();
    println!("{:?}", elapsed);
    println!("Elapsed: {} ms", (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);
    // match start.elapsed() {
    //     Ok(elapsed) => {
    //         println!("Total running time: ~ {} seconds", elapsed.as_secs());
    //     }
    //     Err(_) => {
    //         println!("OUCH!");
    //     }
    // }
}

// It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
fn has_three_vowels(provided_string: &String) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("([aeiouAEIOU])").unwrap();
    }
    let m = RE.find_iter(provided_string);

    return m.count() > 2;
}

// It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
fn two_of_same_letters_in_a_row(provided_string: &String) -> bool {
    let mut previous: char = '%';
    for c in provided_string.chars() {
        if c == previous { return true; } else {previous = c;}
    }

    return false;
}

// It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.
fn illegal_char_group(provided_string: &String) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?i)(ab|cd|pq|xy)").unwrap();
    }
    let is_match = RE.is_match(provided_string);
    return is_match;
}


// Checks if a chair pair exists twice in the string.
fn contains_repeating_char_pair(provided_string: &String) -> bool {
    let mut tmp = provided_string.clone();
    let mut check_for: String = String::new();
    check_for.push(tmp.remove(0));

    while tmp.len() > 2 {
        check_for.push(tmp.remove(0));
        if tmp.contains(&check_for) {
            return true;
        }
        check_for.remove(0);
    }

    return false;
}

// Checks if the pattern [a-z].[a-z] is found in the string.
fn contains_x_something_x (provided_string: &String) -> bool {
    let mut x = '0';
    let mut y = '1';
    
    for (i, c) in provided_string.chars().enumerate() {
        match i {
            1 => x = c,
            2 => y = c,
            _ if c == x => return true,
            _ => {
                x = y;
                y = c;
            }
        }
    }
    return false;
}

// Returns the total result of the part one checks
fn is_string_nice_part_one (provided_string: &String) -> bool {
    return !illegal_char_group(&provided_string) && two_of_same_letters_in_a_row(&provided_string) && has_three_vowels(&provided_string);
}

// Returns the total result of the part two checks
fn is_string_nice_part_two (provided_string: &String) -> bool {
    return contains_repeating_char_pair(&provided_string) && contains_x_something_x(&provided_string);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[2];
    let start = Instant::now();

    // Exercise 5 start:
    let f = File::open(filename).expect("File not found!");
    let f = BufReader::new(f);

    let mut i = 0;
    let mut j = 0;

    for line in f.lines() {
        
        let string_to_check = line.unwrap();

        if is_string_nice_part_one(&string_to_check) { i += 1; }
        if is_string_nice_part_two(&string_to_check) { j += 1; }
        
    }
    
    println!("Nice strings in part one: {}", i);
    println!("Nice strings in part two: {}", j);
    // Exercise 5 end

    time_since_start(start);
}
