use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

fn load_grid(reader: impl BufRead) -> (Vec<Vec<char>>, usize, usize) {
    let grid: Vec<Vec<char>> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.chars().collect())
        .collect();
    
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };
    
    (grid, rows, cols)
}

// bounds checker
fn is_valid(x: i32, y: i32, n: usize) -> bool {
    x >= 0 && y >= 0 && x < n as i32 && y < n as i32
}

fn get_search_directions() -> [(i32, i32); 8] {
    [
        (0, 1),   // Right
        (0, -1),  // Left
        (1, 0),   // Down
        (-1, 0),  // Up
        (1, 1),   // Down-right
        (1, -1),  // Down-left
        (-1, 1),  // Up-right
        (-1, -1), // Up-left
    ]
}

/// - For each character in the word:
///   - Check if the current position is valid using .
///   - Check if the character at `(nx, ny)` in the grid matches the current character in the word.Iterate 
/// - If all characters in the word match, return `true`.
fn check_position(
    grid: &[Vec<char>],
    word: &[char],
    start_x: usize,
    start_y: usize,
    dx: i32,
    dy: i32,
) -> bool {
    let mut nx = start_x as i32;
    let mut ny = start_y as i32;

    for &ch in word {
        if !is_valid(nx, ny, grid.len()) || grid[nx as usize][ny as usize] != ch {
            return false;
        }
        nx += dx;
        ny += dy;
    }
    true
}

/// - Loop through every cell `(x, y)` in the grid:
///   - For each cell, check in all 8 possible directions.
///   - if the word matches starting at `(x, y)` in  direction.
///   - If the word matches, store the starting position `(x, y)` and the direction `(dx, dy)` in the results.
/// - After checking all cells and directions, return a list of all starting positions and directions where the word is found.

fn find_word(grid: &[Vec<char>], word: &str) -> Vec<(usize, usize, i32, i32)> {
    let n = grid.len();
    let word_chars: Vec<char> = word.chars().collect();
    let directions = get_search_directions();
    let mut results = Vec::new();

    for x in 0..n {
        for y in 0..n {
            for &(dx, dy) in &directions {
                if check_position(grid, &word_chars, x, y, dx, dy) {
                    results.push((x, y, dx, dy));
                }
            }
        }
    }

    results
}

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if filename was provided
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    
    let filename = &args[1];

    // Read the file
    let path = Path::new(filename);
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    // get the dimestions and load the data into memory
    let (grid, rows, cols) = load_grid(reader);
    println!("Grid dimensions: {}x{}", rows, cols);

    // Search for the word "XMAS"
    let word = "XMAS";
    //generic implemation of word search in a grid 
    //
    let results = find_word(&grid, word);

   
    let mut output_file = File::create("xmas_positions.txt").expect("Failed to create output file");
    
    
    for (x, y, _, _) in &results {
        writeln!(output_file, "{},{}", x, y).expect("Failed to write to file");
    }
    

    println!("Total occurrences: {}", results.len());
}

