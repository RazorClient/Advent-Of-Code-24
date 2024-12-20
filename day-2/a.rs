use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    
    // Check if filename was provided
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    // Get the filename from arguments
    let filename = &args[1];
    
    // Read the file line by line
    let path = Path::new(filename);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut valid_lines = 0; 

    // Process each line
    for line in reader.lines() {
        let line = line?;
        let numbers: Result<Vec<i32>, _> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>())
            .collect();
        
        match numbers {
            Ok(nums) => {
                if nums.len() < 2 {
                    eprintln!("Error: Each line must contain at least 2 numbers");
                    continue;
                }
            
                if is_valid_sequence(&nums) {
                    valid_lines += 1;
                }
            }
            Err(e) => {
                eprintln!("Error parsing line: {}", e);
                continue;
            }
        }
    }

    println!("Number of valid sequences: {}", valid_lines);
    Ok(())
}

fn is_valid_sequence(nums: &[i32]) -> bool {
    // First check if the sequence is valid without removing any element
    if is_valid_without_removal(nums) {
        return true;
    }


    for skip_idx in 0..nums.len() {
        let mut temp_nums: Vec<i32> = Vec::new();
        for (i, &num) in nums.iter().enumerate() {
            if i != skip_idx {
                temp_nums.push(num);
            }
        }
        if is_valid_without_removal(&temp_nums) {
            return true;
        }
    }
    false
}

fn is_valid_without_removal(nums: &[i32]) -> bool {

    let is_increasing = (0..nums.len()-1).all(|i| {
        let diff = nums[i+1] - nums[i];
        diff >= 1 && diff <= 3
    });
    let is_decreasing = (0..nums.len()-1).all(|i| {
        let diff = nums[i] - nums[i+1];
        diff >= 1 && diff <= 3
    });

    is_increasing || is_decreasing
}
