use std::fs::read_to_string;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn as_char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}

struct Puzzle {
    height: i32,
    width: i32,
    data: Vec<Vec<char>>,
    visited: Vec<Vec<bool>>,
    trace: Vec<Vec<char>>,
    position: (i32, i32),
    cur_dir: Direction,
}

impl Puzzle {
    fn new(height: i32, width: i32, data: Vec<Vec<char>>, position: (i32, i32), cur_dir: Direction) -> Puzzle {
        let mut visited = Vec::new();
        for _ in 0..height {
            visited.push(vec![false; width as usize]);
        }
        visited[position.0 as usize][position.1 as usize] = true;
        let mut trace = Vec::new();
        for _ in 0..height {
            trace.push(vec![' '; width as usize]);
        }
        trace[position.0 as usize][position.1 as usize] = cur_dir.as_char();
        Puzzle {
            height,
            width,
            data,
            visited,
            position,
            cur_dir,
            trace,
        }
    }

    fn clone(&self) -> Puzzle {
        let mut visited = Vec::new();
        for i in 0..self.height {
            visited.push(self.visited[i as usize].clone());
        }
        let mut trace = Vec::new();
        for i in 0..self.height {
            trace.push(self.trace[i as usize].clone());
        }
        let cur_dir = match self.cur_dir {
            Direction::Up => Direction::Up,
            Direction::Down => Direction::Down,
            Direction::Left => Direction::Left,
            Direction::Right => Direction::Right,
        };
        Puzzle {
            height: self.height,
            width: self.width,
            data: self.data.clone(),
            visited,
            position: self.position,
            cur_dir,
            trace,
        }
    }

    fn print(&self) {
        for i in 0..self.height {
            for j in 0..self.width {
                if (i, j) == self.position {
                    print!("{}", self.cur_dir.as_char());
                } else {
                    if self.visited[i as usize][j as usize] {
                        print!("X");
                    } else {
                        print!("{}", self.data[i as usize][j as usize]);
                    }
                }
            }
            println!();
        }
    }

    fn count_visited(&self) -> i32 {
        self.visited.iter().flatten().filter(|&&x| x).count() as i32 + 1
    }

    fn visit(&mut self, i: i32, j: i32) -> bool {
        if self.data[i as usize][j as usize] == '#' {
            self.cur_dir = match self.cur_dir {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            };
            return true;
        }
        if !self.visited[i as usize][j as usize] {
            self.visited[i as usize][j as usize] = true;
            self.data[i as usize][j as usize] = 'X';
        }
        self.position = (i, j);
        if self.trace[i as usize][j as usize] != self.cur_dir.as_char() {
            self.trace[i as usize][j as usize] = self.cur_dir.as_char();
            return true;
        }
        false
    }

    fn next_move(&self) -> (i32, i32) {
        let (i, j) = self.position;
        match self.cur_dir {
            Direction::Up => (i - 1, j),
            Direction::Down => (i + 1, j),
            Direction::Left => (i, j - 1),
            Direction::Right => (i, j + 1),
        }
    }

    fn next_move_within_bounds(&self) -> bool {
        let (i, j) = self.next_move();
        i >= 0 && i < self.height && j >= 0 && j < self.width
    }

    fn solve(&mut self) -> bool {
        while self.next_move_within_bounds() {
            let (i, j) = self.next_move();
            if !self.visit(i, j) {
                return false;
            }
        }
        true
    }

    fn find_glitches(&self) -> u32 {
        let mut glitches = 0;
        for i in 0..self.height {
            for j in 0..self.width {
                let mut test_run = self.clone();
                if test_run.data[i as usize][j as usize] == '.' {
                    test_run.data[i as usize][j as usize] = '#';
                    if !test_run.solve() {
                        glitches += 1;
                    }
                }
            }
        }
        glitches
    }

}

fn main() {
    let input = std::env::args().nth(1).expect("Please provide an input");

    let mut puzzle = parse_input(&input);

    let clone = puzzle.clone();
    puzzle.print();
    puzzle.solve();
    puzzle.print();

    println!("Visited: {}", puzzle.count_visited());
    println!("Glitches: {}", clone.find_glitches());
}

fn parse_input(filename: &str) -> Puzzle {
    let mut data: Vec<Vec<char>> = Vec::new();
    let mut height = 0;
    let mut width = 0;
    let mut position = (0, 0);
    let mut cur_dir = Direction::Up;

    read_to_string(filename).unwrap().lines().enumerate().for_each(|(i, line)| {
        let mut row: Vec<char> = Vec::new();
        line.chars().enumerate().for_each(|(j, c)| {
            row.push(c);
            if c == '^' {
                position = (i as i32, j as i32);
                cur_dir = Direction::Up;
            } else if c == 'v' {
                position = (i as i32, j as i32);
                cur_dir = Direction::Down;
            } else if c == '<' {
                position = (i as i32, j as i32);
                cur_dir = Direction::Left;
            } else if c == '>' {
                position = (i as i32, j as i32);
                cur_dir = Direction::Right;
            }
        });
        data.push(row.clone());
        height += 1;
        width = row.len() as i32;
    });

    Puzzle::new(height, width, data, position, cur_dir)
}