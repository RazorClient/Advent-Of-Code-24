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


/// Checks if the diagonal cells around a given 'A' form an MAS or SAM sequence.
/// Given a center A at (x,y), and diagonal direction offsets (-dx,-dy) and (dx,dy),
/// This function verifies if (x-dx,y-dy) and (x+dx,y+dy) form either M...A...S or S...A...M.
fn check_mas_diagonal(grid: &[Vec<char>], x: usize, y: usize, dx: i32, dy: i32) -> bool {
    let n = grid.len();
    let x1 = x as i32 - dx;
    let y1 = y as i32 - dy;
    let x2 = x as i32 + dx;
    let y2 = y as i32 + dy;

    if !is_valid(x1, y1, n) || !is_valid(x2, y2, n) {
        return false;
    }

    let c1 = grid[x1 as usize][y1 as usize];
    let c2 = grid[x2 as usize][y2 as usize];

    // Valid if they form "M...A...S" or "S...A...M"
    // That means one of them must be 'M' and the other 'S'.
    (c1 == 'M' && c2 == 'S') || (c1 == 'S' && c2 == 'M')
}

/// Find all 'X' patterns formed by two MAS sequences:
/// Each 'X' is formed by:
///   Top-left and bottom-right diagonal: M-A-S or S-A-M
///   Top-right and bottom-left diagonal: M-A-S or S-A-M
fn find_x_mas_patterns(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let n = grid.len();
    let mut results = Vec::new();

    for x in 0..n {
        for y in 0..n {
            if grid[x][y] == 'A' {
                // Check the two diagonals for MAS patterns
                // Diagonal 1: top-left (x-1, y-1) and bottom-right (x+1, y+1)
                // Diagonal 2: top-right (x-1, y+1) and bottom-left (x+1, y-1)
                let diag1_ok = check_mas_diagonal(grid, x, y, 1, 1);  // dx=1,dy=1 checks top-left and bottom-right
                let diag2_ok = check_mas_diagonal(grid, x, y, 1, -1); // dx=1,dy=-1 checks top-right and bottom-left

                if diag1_ok && diag2_ok {
                    results.push((x, y));
                }
            }
        }
    }

    results
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let path = Path::new(filename);
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let (grid, rows, cols) = load_grid(reader);
    println!("Grid dimensions: {}x{}", rows, cols);

    // Remove XMAS word search and output file creation
    // Focus only on X patterns
    let x_patterns = find_x_mas_patterns(&grid);
    println!("Found {} 'X' MAS patterns:", x_patterns.len());
    for (x, y) in x_patterns {
        println!("X pattern center at: {},{}", x, y);
    }
}

