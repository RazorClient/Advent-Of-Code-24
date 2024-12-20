use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

fn parse_line(line: &str, enabled: &mut bool) -> Vec<(i32, i32)> {
    let mut results = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        // Check for "do("
        if i + 3 <= chars.len() && &chars[i..(i+3)] == ['d', 'o', '('] {
            // Found do()
            *enabled = true;
            i += 3;
            continue;
        }

        // Check for "don't("
        // We expect 'd', 'o', 'n', ''', 't', '(' in sequence
        if i + 6 <= chars.len() 
            && chars[i] == 'd' 
            && chars[i+1] == 'o' 
            && chars[i+2] == 'n' 
            && chars[i+3] == '\'' 
            && chars[i+4] == 't' 
            && chars[i+5] == '(' {
            // Found don't()
            *enabled = false;
            i += 6;
            continue;
        }

        // Look for "mul("
        if i + 4 <= chars.len() 
            && chars[i] == 'm' 
            && chars[i+1] == 'u' 
            && chars[i+2] == 'l' 
            && chars[i+3] == '(' {

            i += 4; // Move past "mul("
            let mut num1_str = String::new();
            let mut num2_str = String::new();

            // Get first number
            while i < chars.len() && chars[i].is_ascii_digit() {
                num1_str.push(chars[i]);
                i += 1;
            }

            // Check for comma
            if i < chars.len() && chars[i] == ',' {
                i += 1;

                // Get second number
                while i < chars.len() && chars[i].is_ascii_digit() {
                    num2_str.push(chars[i]);
                    i += 1;
                }

                if i < chars.len() && chars[i] == ')' {
                    // Parse numbers and validate
                    if let (Ok(x), Ok(y)) = (num1_str.parse::<i32>(), num2_str.parse::<i32>()) {
                        if (1..=999).contains(&x) && (1..=999).contains(&y) {
                            // Only add results if currently enabled
                            if *enabled {
                                results.push((x, y));
                            }
                        }
                    }
                }
            }
            i += 1;
            continue;
        }

        i += 1;
    }

    results
}

fn process_file(input_path: &str, output_path: &str) -> io::Result<i32> {
    let mut total_sum = 0;
    let mut enabled = true; // At the beginning of the program, mul is enabled.

    // Open input file for reading
    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);

    // Open output file for writing
    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output_path)?;

    // Process each line
    for line in reader.lines() {
        let line = line?;
        let multiplications = parse_line(&line, &mut enabled);

        // If we found any valid (and enabled) multiplications
        if !multiplications.is_empty() {
            // Add all multiplications to total
            for (x, y) in &multiplications {
                total_sum += x * y;
            }
            // Write only the mul expressions to the output file
            let mul_expressions = multiplications.iter()
                .map(|(x, y)| format!("mul({},{})", x, y))
                .collect::<Vec<String>>()
                .join(" ");
            writeln!(output_file, "{}", mul_expressions)?;
        }
    }

    Ok(total_sum)
}

fn main() -> io::Result<()> {

    let args: Vec<String> = std::env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }
    
    let input_file = &args[1];
    let output_file = "output.txt";
    
    match process_file(input_file, output_file) {
        Ok(sum) => println!("Total sum of all multiplications: {}", sum),
        Err(e) => eprintln!("Error processing file: {}", e),
    }

    Ok(())
}
