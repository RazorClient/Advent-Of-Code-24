use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let mut vec_a: Vec<i32> = Vec::new();
    let mut vec_b: Vec<i32> = Vec::new();

    // Read the file line by line
    if let Ok(lines) = read_lines("a.txt") {
        for line in lines {
            if let Ok(content) = line {
                let parts: Vec<&str> = content.split_whitespace().collect();
                if parts.len() >= 2 {
                    // Convert strings to integers
                    if let (Ok(num_a), Ok(num_b)) = (parts[0].parse(), parts[1].parse()) {
                        vec_a.push(num_a);
                        vec_b.push(num_b);
                    }
                }
            }
        }
    }

    // Sort both vectors
    vec_a.sort();
    vec_b.sort();

    // Calculate total distance between paired numbers
    let total_distance: i32 = vec_a.iter()
        .zip(vec_b.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("Total distance between paired numbers: {}", total_distance);

    Ok(())
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
