use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
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
    // get to know what we are working with here
    
    let (grid, rows, cols) = load_grid(reader);
    println!("Grid dimensions: {}x{}", rows, cols);

    // Remove XMAS word search and output file creation
    // Focus only on X patterns
    let x_patterns = find_x_mas_patterns(&grid);
    println!("Found {} 'X' MAS patterns:", x_patterns.len());

}

