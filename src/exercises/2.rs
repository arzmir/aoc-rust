use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let input_filename = "./inputs/2a.txt";
    let f = File::open(input_filename).unwrap();

    let mut total_area = 0;
    let mut total_ribbon = 0;

    for line in BufReader::new(f).lines() {
        let line_text = line.unwrap();
        let mut a = line_text.split("x").map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        
        a.sort();
        total_area += 3*a[0]*a[1] + 2*a[1]*a[2] + 2*a[2]*a[0];
        total_ribbon += 2*(a[0]+a[1]) + a[0]*a[1]*a[2];
        
    }

    println!("Total wrapping paper needed {} sqft.", total_area);
    println!("Total ribbon needed {} ft.", total_ribbon);
}
