extern crate md5;

use md5::compute;

fn four (puzzle_input: &str, zeroes: usize) {
    let mut counter = 1;
    let mut not_found = true;

    while not_found {
        let current_digest = format!("{:x}", compute(format!("{}{}", puzzle_input, counter)));
        let first_x = current_digest.chars().take(zeroes).collect::<Vec<char>>();
        let mut digits_only = true;

        for c in &first_x {
            match c.to_digit(10) {
                Option::None => { digits_only = false; break; },
                _ => continue,
            }
        }

        if digits_only {
            if &first_x.iter().map(|n| n.to_digit(10).unwrap()).sum::<u32>() == &0u32 {
                println!("Counter: {} -- {:?}", counter, first_x);
                not_found = false;
                break;
            }
        }

        counter += 1;
    }


}

fn main() {
    let puzzle_input = "ckczppom";

    four(puzzle_input, 5usize);
    four(puzzle_input, 6usize);
}
