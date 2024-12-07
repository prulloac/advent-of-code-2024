use std::fs::read_to_string;

struct Matrix {
    rows: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

fn main() {
    let input = std::env::args().nth(1).expect("Please provide an input");

    let matrix = parse_input(&input);

    for row in 0..matrix.height {
        for col in 0..matrix.width {
            print!("{}", matrix.rows[row][col]);
        }
        print!("\n");
    }

    let search = "XMAS";
    let xmas_counter = search_in_matrix(&matrix, &search);

    println!("Search string: {}", search);
    println!("Found: {}", xmas_counter);
    println!("---------------------");

    let x_mas_counter = search_x_mas_in_matrix(&matrix);
    println!("Found: {} X-MAS", x_mas_counter);
}


fn parse_input(filename: &str) -> Matrix {
    let mut rows: Vec<Vec<char>> = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        rows.push(row);
    }
    let height = rows.len();
    let width = rows.get(0).unwrap().len();
    Matrix { rows, height, width }
}

fn search_in_matrix(matrix: &Matrix, search: &str) -> u32 {
    let mut counter = 0;
    for row in 0..matrix.height {
        for col in 0..matrix.width {
            if matrix.rows[row][col] == search.chars().next().unwrap() {
                counter += search_matches_from_position(matrix, search, row, col);
            }
        }
    }
    counter
}

fn search_matches_from_position(matrix: &Matrix, search: &str, row_number: usize, col_number: usize) -> u32 {
    let mut counter = 0;
    for x_axis in -1..2 {
        for y_axis in -1..2 {
            if x_axis == 0 && y_axis == 0 {
                continue;
            }
//            println!("Position: ({}, {}), Direction: ({}, {})", row_number, col_number, x_axis, y_axis);
            for step in 0..search.len() {
                let row = row_number as i32 + y_axis * step as i32;
                let col = col_number as i32 + x_axis * step as i32;
                if !valid_position(matrix, row as usize, col as usize) {
//                        println!("FLAG Invalid. Position: ({}, {}), Direction: ({}, {}), At: ({}, {}), Step: {}", row_number, col_number, x_axis, y_axis, row, col, step);
                    break;
                }
                let char = matrix.rows[row as usize][col as usize];
                if step == search.len() - 1 && char == search.chars().nth(step).unwrap() {
//                    println!("FLAG Found. Position: ({}, {}), Direction: ({}, {}), Char: {}", row_number, col_number, x_axis, y_axis, char);
                    counter += 1;
                } else {
//                    println!("FLAG Checking. Position: ({}, {}), Direction: ({}, {}), At: ({}, {}), Step: {}", row_number, col_number, x_axis, y_axis, row, col, step);
                    if char != search.chars().nth(step).unwrap() {
//                        println!("FLAG Mismatch. Position: ({}, {}), Direction: ({}, {}), At: ({}, {}), Step: {}, Char: {}", row_number, col_number, x_axis, y_axis, row, col, step, matrix.rows[row as usize][col as usize]);
                        break;
                    }    
                }
            }
        }
    }
    if counter > 0 {
//        println!("Position: ({}, {}), Counter: {}", row_number, col_number, counter);
    }
    counter
}

fn valid_position(matrix: &Matrix, row: usize, col: usize) -> bool {
    row < matrix.height && col < matrix.width
}

fn search_x_mas_in_matrix(matrix: &Matrix) -> u32 {
    let mut counter = 0;
    for row in 1..matrix.height-1 {
        for col in 1..matrix.width-1 {
            if matrix.rows[row][col] == 'A' {
//                println!("Found A at ({}, {})", row, col);
                let mut xmas = true;
                // check first diagonal (top-left to bottom-right) M-A-S or S-A-M
                if !((matrix.rows[row-1][col-1] == 'M' && matrix.rows[row+1][col+1] == 'S') || (matrix.rows[row-1][col-1] == 'S' && matrix.rows[row+1][col+1] == 'M')) {
//                    println!("First diagonal failed");
                    xmas = false;
                }
                // check second diagonal (top-right to bottom-left) M-A-S or S-A-M
                if !((matrix.rows[row-1][col+1] == 'M' && matrix.rows[row+1][col-1] == 'S') || (matrix.rows[row-1][col+1] == 'S' && matrix.rows[row+1][col-1] == 'M')) {
//                    println!("Second diagonal failed");
                    xmas = false;
                }
                if xmas {
                    counter += 1;
                }
            }
        }
    }
    counter
}