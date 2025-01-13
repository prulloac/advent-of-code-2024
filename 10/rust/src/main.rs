use std::{env::args, fs::read_to_string, fmt};

#[derive(Clone)]
struct Trailhead {
    start: Coordinate,
    score: usize,
    rating: usize,
    peaks: Vec<Coordinate>,
    routes: Vec<Vec<Coordinate>>,
}

impl fmt::Debug for Trailhead {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Trailhead: {:?}, Score: {}, Rating: {}", self.start, self.score, self.rating)
    }
}

impl Trailhead {
    fn new(start: Coordinate) -> Self {
        Self {
            start,
            score: 0,
            rating: 0,
            peaks: Vec::new(),
            routes: Vec::new(),
        }
    }

    fn calc_score(&mut self, start: &Coordinate, map: &Map, history: Vec<Coordinate>) {
        let reachables: Vec<Coordinate> = self.find_reachables(start, map.height, map.width);
        let me = map.input[start.y][start.x].to_digit(10).unwrap() as i32;
        for pos in reachables.iter() {
            let neighbour = map.input[pos.y][pos.x].to_digit(10).unwrap() as i32;
            if neighbour - me == 1 {
                let mut new_history = history.clone();
                if neighbour == 9{
                    if !self.peaks.contains(pos) {
                        new_history.push(pos.clone());
                        self.peaks.push(pos.clone());
                        self.routes.push(new_history.clone());
                        self.score += 1;
                    }
                    continue;
                }
                new_history.push(pos.clone());
                self.calc_score(pos, map, new_history);
            }
        }
    }

    fn calc_rating(&mut self, start: &Coordinate, map: &Map, history: Vec<Coordinate>) {
        let reachables: Vec<Coordinate> = self.find_reachables(start, map.height, map.width);
        let me = map.input[start.y][start.x].to_digit(10).unwrap() as i32;
        for pos in reachables.iter() {
            let neighbour = map.input[pos.y][pos.x].to_digit(10).unwrap() as i32;
            if neighbour - me == 1 {
                let mut new_history = history.clone();
                if neighbour == 9{
                    self.rating += 1;
                }
                new_history.push(pos.clone());
                self.calc_rating(pos, map, new_history);
            }
        }
    }

    fn find_reachables(&self, coord: &Coordinate, height: usize, width: usize) -> Vec<Coordinate> {
        let mut reachables = Vec::new();
        for y in -1..2 {
            for x in -1..2 {
                if x == y || x == -y { // to avoid diagonal moves
                    continue;
                }
                let new_x = coord.x as i32 + x;
                let new_y = coord.y as i32 + y;
                if new_x >= 0 && new_x < width as i32 && new_y >= 0 && new_y < height as i32 {
                    reachables.push(Coordinate::new(new_x as usize, new_y as usize));
                }
            }
        }
        return reachables;
    }

}

#[derive(Clone)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl fmt::Debug for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{row: {}, col: {}}}", self.y+1, self.x+1)
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

struct Map {
    input: Vec<Vec<char>>,
    width: usize,
    height: usize,
    trailheads: Vec<Trailhead>,
    scores: Vec<usize>,
    ratings: Vec<usize>,
}

impl Map {
    fn new(input: Vec<Vec<char>>) -> Self {
        let width = input[0].len();
        let height = input.len();
        let trailheads = Vec::new();
        let scores = Vec::new();
        let ratings = Vec::new();
        Self {
            input,
            width,
            height,
            trailheads,
            scores,
            ratings
        }
    }

    fn print(&self) {
        self.print_map();
        self.print_trailheads();
        println!("Total score: {}", self.scores.iter().sum::<usize>());
        println!("Total rating: {}", self.ratings.iter().sum::<usize>());
    }

    fn print_map(&self) {
        println!("Map:");
        for row in self.input.iter() {
            println!("{}", row.iter().collect::<String>());
        }
    }

    fn print_trailheads(&self) {
        for t in self.trailheads.iter() {
            println!("{:?}", t);
        }
    }

    fn find_zeros(&mut self) {
        self.trailheads.clear();
        let mut zeros: Vec<Trailhead> = Vec::new();
        for (y, row) in self.input.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == '0' {
                    zeros.push(Trailhead::new(Coordinate::new(x, y)));
                }
            }
        }
        self.trailheads.append(&mut zeros);
    }

    fn find_trails(&mut self) {
        self.scores.clear();
        for idx in 0..self.trailheads.len() {
            let mut trailhead = self.trailheads[idx].clone();
            let start = trailhead.start.clone();
            let mut history = Vec::new();
            history.push(start.clone());
            trailhead.calc_score(&start, self, history.clone());
            trailhead.calc_rating(&start, self, history.clone());
            self.scores.push(trailhead.score);
            self.ratings.push(trailhead.rating);
            self.trailheads[idx] = trailhead;
        }
    }
}

fn main() {
    let filename = args().nth(1).expect("Input file name missing");
    let mut map = read_map(&filename);
    map.find_zeros();
    map.find_trails();
    map.print();
}

fn read_map(filename: &str) -> Map {
    let input = read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    Map::new(input)
}
