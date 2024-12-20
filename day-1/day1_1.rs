use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let mut vec_a: Vec<i32> = Vec::new();
    let mut vec_b: Vec<i32> = Vec::new();

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


    // Calculate frequency of numbers in vec_b
    let mut frequency_map: HashMap<i32, i32> = HashMap::new();
    for &num in &vec_b {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    // Calculate similarity score
    let similarity_score: i32 = vec_a.iter()
        .map(|&num| num * frequency_map.get(&num).unwrap_or(&0))
        .sum();

    println!("Similarity score: {}", similarity_score);

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
