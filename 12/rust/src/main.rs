use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl std::fmt::Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Direction::North => write!(f, "North"),
            Direction::East => write!(f, "East"),
            Direction::South => write!(f, "South"),
            Direction::West => write!(f, "West"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Cell {
    row: usize,
    col: usize,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(row: {}, col: {})", self.row, self.col)
    }
}

impl Cell {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn sides(&self) -> Vec<Side> {
        let mut sides = vec![];
        sides.push(Side {
            dir: Direction::North,
            cell: &self,
        });
        sides.push(Side {
            dir: Direction::East,
            cell: &self,
        });
        sides.push(Side {
            dir: Direction::South,
            cell: &self,
        });
        sides.push(Side {
            dir: Direction::West,
            cell: &self,
        });
        return sides;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Side<'a> {
    dir: Direction,
    cell: &'a Cell,
}

impl<'a> std::fmt::Debug for Side<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?} -> {:?}", self.cell, self.dir)
    }
}

impl<'a> Side<'a> {
    fn continuation(&self, other: &Self) -> bool {
        let same_dir = self.dir == other.dir;
        let same_col = self.cell.col == other.cell.col;
        let same_row = self.cell.row == other.cell.row;
        let neighbor_col = self.cell.col == other.cell.col + 1
            || self.cell.col as i32 == other.cell.col as i32 - 1;
        let neighbor_row = self.cell.row == other.cell.row + 1
            || self.cell.row as i32 == other.cell.row as i32 - 1;
        let vertical_continuation = same_col
            && neighbor_row
            && same_dir
            && (self.dir == Direction::East || self.dir == Direction::West);
        let horizontal_continuation = same_row
            && neighbor_col
            && same_dir
            && (self.dir == Direction::North || self.dir == Direction::South);
        return vertical_continuation || horizontal_continuation;
    }

    fn same_border(&self, other: &Self) -> bool {
        let same_col = self.cell.col == other.cell.col;
        let same_row = self.cell.row == other.cell.row;
        let west_east_neighbor = self.dir == Direction::East
            && other.dir == Direction::West
            && same_row
            && other.cell.col as i32 - self.cell.col as i32 == 1;
        let east_west_neighbor = self.dir == Direction::West
            && other.dir == Direction::East
            && same_row
            && self.cell.col as i32 - other.cell.col as i32 == 1;
        let north_south_neighbor = self.dir == Direction::South
        && other.dir == Direction::North 
            && same_col
            && other.cell.row as i32 - self.cell.row as i32 == 1;
        let south_north_neighbor = self.dir == Direction::North
        && other.dir == Direction::South
            && same_col
            && self.cell.row as i32 - other.cell.row as i32 == 1;
        return west_east_neighbor
            || east_west_neighbor
            || north_south_neighbor
            || south_north_neighbor;
    }
}

struct Plot {
    perimeter: u32,
    area: u32,
    symbol: char,
    cells: Vec<Cell>,
    sides: u32,
}

impl Plot {
    fn value(&self) -> u32 {
        self.area * self.perimeter
    }

    fn new(symbol: char) -> Self {
        Self {
            perimeter: 0,
            area: 0,
            sides: 0,
            symbol,
            cells: vec![],
        }
    }
}

struct Garden {
    input: Vec<Vec<char>>,
    visited: Vec<Vec<bool>>,
    plots: Vec<Plot>,
    width: usize,
    height: usize,
}

impl std::fmt::Display for Garden {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Garden:")?;
        for row in &self.input {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "Plots: {}", self.plots.len())?;
        for (_, plot) in self.plots.iter().enumerate() {
            writeln!(
                f,
                "A region of {} plants with price {} * {} = {}",
                plot.symbol,
                plot.area,
                plot.perimeter,
                plot.value()
            )?;
            writeln!(f, "Cells: {:?}", plot.cells)?;
            writeln!(f, "Sides: {:?}", plot.sides)?;
        }
        let value_1: u32 = self.plots.iter().fold(0, |acc, plot| acc + plot.value());
        writeln!(f, "Total Garden Value by formula 1: {}", value_1)?;
        let value_2: u32 = self.plots.iter().map(|plot| plot.area * plot.sides).sum();
        writeln!(f, "Total Garden Value by formula 2: {}", value_2)?;
        Ok(())
    }
}

impl Garden {
    fn new(input: Vec<Vec<char>>) -> Self {
        let height = input.len();
        let width = input[0].len();
        let visited = vec![vec![false; width]; height];
        let plots = vec![];
        Self {
            input,
            visited,
            plots,
            width,
            height,
        }
    }

    fn estimate_plots(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                if !self.visited[row][col] {
                    let plot: Plot = self.measure_plot(row, col);
                    self.plots.push(plot);
                }
            }
        }
    }

    fn measure_plot(&mut self, row: usize, col: usize) -> Plot {
        let mut perimeter = 0;
        let mut area = 0;
        let mut cells = vec![(row as i32, col as i32)];
        let symbol = self.input[row][col];
        let mut plot = Plot::new(symbol);
        while let Some((y, x)) = cells.pop() {
            if y < 0 || y as usize >= self.height || x < 0 || x as usize >= self.width {
                perimeter += 1;
                continue;
            }
            let row = y as usize;
            let col = x as usize;
            if self.input[row][col] != symbol {
                perimeter += 1;
                continue;
            }
            if self.visited[row][col] {
                continue;
            }
            let cell = Cell::new(row, col);
            plot.cells.push(cell);
            area += 1;
            self.visited[row][col] = true;
            cells.push((y - 1, x));
            cells.push((y + 1, x));
            cells.push((y, x - 1));
            cells.push((y, x + 1));
        }
        plot.perimeter = perimeter;
        plot.area = area;
        plot.sides = self.group_sides(plot.cells.clone());
        return plot;
    }

    fn group_sides(&self, cells: Vec<Cell>) -> u32 {
        // fill sides
        let mut sides = cells.iter().map(|cell| cell.sides()).flatten().collect_vec();
        sides.sort_by(|a,b| a.dir.cmp(&b.dir).then(a.cell.row.cmp(&b.cell.row)).then(a.cell.col.cmp(&b.cell.col)));
        // remove when same border
        let mut i = 0;
        while i < sides.len() {
            if sides.iter().any(|s| sides[i].same_border(s)) {
                let to_remove = sides.iter().find_position(|s| sides[i].same_border(s)).unwrap();
                println!("Same border detected: {:?} -> {:?}", sides[i], to_remove.1);
                sides.remove(to_remove.0);
                sides.remove(i);
            } else {
                i += 1;
            }
        }
        // remove when continuation
        let mut i = 0;
        while i < sides.len() {
            if sides.iter().any(|s| sides[i].continuation(s)) {
                let to_remove = sides.iter().find_position(|s| sides[i].continuation(s)).unwrap();
                println!("Continuation detected: {:?} -> {:?}", sides[i], to_remove.1);
                sides.remove(i);
            } else {
                i += 1;
            }
        }
        let sample = cells[0];
        println!("Plot: {}", self.input[sample.row][sample.col]);
        println!("Cells: {:?}", cells);
        println!("Sides ({}):", sides.len());
        sides.iter().for_each(|side| println!("{:?}", side));
        println!();
        return sides.len() as u32;
    }
}

fn main() {
    let filename = std::env::args().nth(1).expect("Filename required");
    let mut garden: Garden = Garden::new(
        std::fs::read_to_string(filename)
            .expect("Failed to read file")
            .lines()
            .map(|line| line.chars().collect())
            .collect(),
    );
    garden.estimate_plots();
    println!("{}", garden);
}
