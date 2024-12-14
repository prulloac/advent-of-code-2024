use std::{fs::read_to_string, collections::{HashMap, HashSet}};

#[derive(Debug)]
struct Antena {
    frequency: char,
    row: usize,
    col: usize,
}

struct Chart {
    rows: usize,
    cols: usize,
    antenas: HashMap<char, Vec<Antena>>,
    anti_nodes: HashSet<(usize, usize)>,
    anti_nodes_v2: HashSet<(usize, usize)>,
    map: Vec<Vec<char>>,
}

impl Chart {
    fn new(map: Vec<Vec<char>>) -> Self {
        let rows = map.len();
        let cols = map[0].len();
        let mut antenas: HashMap<char, Vec<Antena>> = HashMap::new();
        let anti_nodes = HashSet::new();
        let anti_nodes_v2 = HashSet::new();
        for (i, row) in map.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if *c == '.' {
                    continue;
                }
                if antenas.contains_key(c) {
                    antenas.get_mut(c).unwrap().push(Antena::new(*c, i, j));
                } else {
                    antenas.insert(*c, vec![Antena::new(*c, i, j)]);
                }
            }
        }
        Self {
            rows,
            cols,
            antenas,
            anti_nodes,
            map,
            anti_nodes_v2,
        }
    }
    
    fn find_anti_nodes(&mut self) {
        let within_boundaries = |row, col| row >= 0 && row < self.rows as isize && col >= 0 && col < self.cols as isize;
        for frequency in self.antenas.keys() {
            let antenas = self.antenas.get(frequency).unwrap();
            for (i, antena) in antenas.iter().enumerate() {
                for (j, other_antena) in antenas.iter().enumerate() {
                    if i == j {
                        continue;
                    }
                    println!("Measuring Antena {} to Antena {}", antena, other_antena);
                    // both antenas are automatically anti nodes v2
                    self.anti_nodes_v2.insert((antena.row, antena.col));
                    self.anti_nodes_v2.insert((other_antena.row, other_antena.col));
                    // distance between two antenas per axis
                    let row_distance = antena.row as isize - other_antena.row as isize;
                    let col_distance = antena.col as isize - other_antena.col as isize;
                    // add the distance to the other antena
                    let anti_node_a_row = other_antena.row as isize - row_distance;
                    let anti_node_a_col = other_antena.col as isize - col_distance;
                    let anti_node_b_row = antena.row as isize + row_distance;
                    let anti_node_b_col = antena.col as isize + col_distance;
                    // check if the anti nodes are within the bounds of the map
                    if within_boundaries(anti_node_a_row,anti_node_a_col) {
                        println!("Found anti node at ({}, {})", anti_node_a_row, anti_node_a_col);
                        self.anti_nodes.insert((anti_node_a_row as usize, anti_node_a_col as usize));
                        self.anti_nodes_v2.insert((anti_node_a_row as usize, anti_node_a_col as usize));
                    }
                    if within_boundaries(anti_node_b_row, anti_node_b_col) {
                        println!("Found anti node at ({}, {})", anti_node_b_row, anti_node_b_col);
                        self.anti_nodes.insert((anti_node_b_row as usize, anti_node_b_col as usize));
                        self.anti_nodes_v2.insert((anti_node_b_row as usize, anti_node_b_col as usize));
                    }
                    // for anti node v2 we need to continue calculating same distances until we reach the edge of the map on both directions
                    let mut anti_node_v2_a_row = anti_node_a_row;
                    let mut anti_node_v2_a_col = anti_node_a_col;
                    // first direction is up and left
                    while within_boundaries(anti_node_v2_a_row, anti_node_v2_a_col) {
                        if anti_node_v2_a_row < self.rows as isize && anti_node_v2_a_col < self.cols as isize {
                            println!("Found anti node v2 at ({}, {})", anti_node_v2_a_row, anti_node_v2_a_col);
                            self.anti_nodes_v2.insert((anti_node_v2_a_row as usize, anti_node_v2_a_col as usize));
                        }
                        anti_node_v2_a_row -= row_distance;
                        anti_node_v2_a_col -= col_distance;
                    }
                    anti_node_v2_a_row = anti_node_a_row;
                    anti_node_v2_a_col = anti_node_a_col;
                    // second direction is down and right
                    while within_boundaries(anti_node_v2_a_row, anti_node_v2_a_col) {
                        if anti_node_v2_a_row >= 0 && anti_node_v2_a_col >= 0 {
                            println!("Found anti node v2 at ({}, {})", anti_node_v2_a_row, anti_node_v2_a_col);
                            self.anti_nodes_v2.insert((anti_node_v2_a_row as usize, anti_node_v2_a_col as usize));
                        }
                        anti_node_v2_a_row += row_distance;
                        anti_node_v2_a_col += col_distance;
                    }
                }
            }
        }
    }

    fn print_map(&self) {
        for row in self.map.iter() {
            for c in row.iter() {
                print!("{}", c);
            }
            println!();
        }
    }

    fn print_with_anti_nodes(&self) {
        let mut map = self.map.clone();
        for (row, col) in self.anti_nodes.iter() {
            if map[*row][*col] == '.' {
                map[*row][*col] = '#';
            }
        }
        for row in map.iter() {
            for c in row.iter() {
                print!("{}", c);
            }
            println!();
        }
    }
}

impl Antena {
    fn new(frequency: char, row: usize, col: usize) -> Self {
        Self {
            frequency,
            row,
            col,
        }
    }
}

impl std::fmt::Display for Antena {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Frequency: {}, Row: {}, Col: {}", self.frequency, self.row, self.col)
    }
    
}

fn main() {
    let input = std::env::args().nth(1).expect("Please provide an input");
    let contents = parse_input(&input);
    let mut chart = Chart::new(contents.iter().map(|line| line.chars().collect()).collect());
    println!("Map:");
    chart.print_map();
    chart.find_anti_nodes();
    println!("Map with anti nodes:");
    chart.print_with_anti_nodes();
    println!("Total anti nodes: {}", chart.anti_nodes.len());
    println!("Total anti nodes v2: {}", chart.anti_nodes_v2.len());

}

fn parse_input(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .expect("Failed to read file")
        .trim()
        .split("\n")
        .map(|line| line.to_string())
        .collect()
}
