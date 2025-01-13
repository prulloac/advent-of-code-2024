struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
        }
    }
}

struct Button {
    name: String,
    point: Point,
}

impl std::fmt::Display for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Button {}: X+{}, Y+{}", self.name, self.point.x, self.point.y)
    }
}

impl Button {
    fn new(name: String, point: Point) -> Self {
        Self {
            name,
            point,
        }
    }
}

impl From<String> for Button {
    fn from(input: String) -> Self {
        let parts: Vec<&str> = input.split(": ").collect();
        let name = parts[0].split_whitespace().collect::<Vec<&str>>()[1].to_string();
        let coords = parts[1].split(", ").collect::<Vec<&str>>();
        let x = coords[0].replace("X+", "").parse::<f64>().unwrap();
        let y = coords[1].replace("Y+", "").parse::<f64>().unwrap();
        let point = Point::new(x, y);
        Button::new(name, point)
    }
}

struct ClawMachine {
    button_a: Button,
    button_b: Button,
    prize: Point,
}

impl std::fmt::Display for ClawMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ClawMachine\n{}\n{}\nPrize: X={}, Y={}", self.button_a, self.button_b, self.prize.x, self.prize.y)
    }
}

impl ClawMachine {
    fn new(button_a: Button, button_b: Button, prize: Point) -> Self {
        Self {
            button_a,
            button_b,
            prize,
        }
    }
}

fn main() {
    let filename = std::env::args().nth(1).expect("No filename provided");
    let buton_a_cost = std::env::args().nth(2).expect("No button A cost provided").parse::<f64>().unwrap();
    let buton_b_cost = std::env::args().nth(3).expect("No button B cost provided").parse::<f64>().unwrap();
    let shift: f64 = std::env::args().nth(4).expect("No shift provided").parse::<f64>().unwrap();
    let contents = std::fs::read_to_string(filename)
        .expect("Cannot read file")
        .lines()
        .map(|x| x.to_string())
        .filter(|x| x.len() > 0)
        .collect::<Vec<String>>();
    let mut i = 0;
    let mut claw_machines: Vec<ClawMachine> = Vec::new();
    while i < contents.len() {
        let button_a = Button::from(contents.get(i).unwrap().to_string());
        let button_b = Button::from(contents.get(i + 1).unwrap().to_string());
        let prize_coordinates = contents.get(i+2).unwrap().split(": ").collect::<Vec<&str>>()[1].split(", ").collect::<Vec<&str>>();
        let prize = Point::new(
            prize_coordinates[0].replace("X=", "").parse::<f64>().unwrap()+shift, 
            prize_coordinates[1].replace("Y=", "").parse::<f64>().unwrap()+shift);
        let claw_machine = ClawMachine::new(button_a, button_b, prize);
        claw_machines.push(claw_machine);
        i += 3
    }
    for (idx, claw_machine) in claw_machines.iter().clone().enumerate() {
        println!("{}: {}\n", idx, claw_machine);
    }

    println!("Button A cost: {}", buton_a_cost);
    println!("Button B cost: {}", buton_b_cost);
    let tokens = calculate_tokens(claw_machines, buton_a_cost, buton_b_cost);
    println!("Tokens: {}", tokens);
}

fn calculate_tokens(claw_machines: Vec<ClawMachine>, button_a_cost: f64, button_b_cost: f64) -> u64 {
    let mut tokens: u64 = 0;
    // solve using linear algebra
    for (idx, claw_machine) in claw_machines.iter().clone().enumerate() {
        let ax: f64 = claw_machine.button_a.point.x;
        let ay: f64 = claw_machine.button_a.point.y;
        let bx: f64 = claw_machine.button_b.point.x;
        let by: f64 = claw_machine.button_b.point.y;
        let px: f64 = claw_machine.prize.x;
        let py: f64 = claw_machine.prize.y;
        // ax * i + bx * j = px
        // ay * i + by * j = py
        // solve for i and j
        let det: f64 = (ax * by - ay * bx) as f64;
        if det == 0.0 {
            println!("No solution for claw machine: {}", idx);
            continue;
        }
        let i: f64 = (px * by - py * bx) as f64 / det;
        let j: f64 = (px as f64 - ax as f64 * i) / bx as f64;
        if i.rem_euclid(1.0) != 0.0 || j.rem_euclid(1.0) != 0.0 {// || i > 100.0 || j > 100.0 {
            println!("No integer solution for claw machine: {}", idx);
            continue;
        }
        tokens += i as u64 * button_a_cost as u64 + j as u64 * button_b_cost as u64;
        println!("Claw machine {}: A={} times, B={} times, tokens={}", idx, i, j, i as u64 * button_a_cost as u64 + j as u64 * button_b_cost as u64);
    }
    tokens
}
