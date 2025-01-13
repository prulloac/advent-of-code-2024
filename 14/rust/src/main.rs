use std::clone::{self, Clone};

#[derive(Clone, PartialEq, Eq)]
struct Robot {
    x: u32,
    y: u32,
    v_x: i32,
    v_y: i32,
}

impl From<&str> for Robot {
    fn from(input: &str) -> Self {
        let preformat = input
        .replace("p=", "")
        .replace("v=", "")
        .replace(",", " ");
        let parts: Vec<&str> = preformat.split_whitespace().collect();
        let x = parts[0].parse::<u32>().unwrap();
        let y = parts[1].parse::<u32>().unwrap();
        let v_x = parts[2].parse::<i32>().unwrap();
        let v_y = parts[3].parse::<i32>().unwrap();
        Robot {
            x,
            y,
            v_x,
            v_y,
        }
    }
}

#[derive(Clone)]
struct Room {
    width: u32,
    height: u32,
    robots: Vec<Robot>,
}

impl std::fmt::Display for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut grid: Vec<Vec<i32>> = vec![vec![0; self.width as usize]; self.height as usize];
        for robot in &self.robots {
            grid[robot.y as usize][robot.x as usize] += 1;
        }
        for row in grid {
            for cell in row {
                if cell == 0 {
                    write!(f, ".")?;
                } else {
                    write!(f, "{}", cell)?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut grid: Vec<Vec<i32>> = vec![vec![0; self.width as usize]; self.height as usize];
        for robot in &self.robots {
            write!(f, "p={},{} v={},{}\n", robot.x, robot.y, robot.v_x, robot.v_y)?;
            grid[robot.y as usize][robot.x as usize] += 1;
        }
        for row in grid {
            for cell in row {
                if cell == 0 {
                    write!(f, ".")?;
                } else {
                    write!(f, "{}", cell)?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Room {
    fn new(width: u32, height: u32, robots: Vec<Robot>) -> Self {
        Room {
            width,
            height,
            robots,
        }
    }

    fn tick(&mut self) {
        for robot in &mut self.robots {
            // x = (x + v_x + width) % width
            // y = (y + v_y + height) % height
            robot.x = (robot.x as i32 + robot.v_x + self.width as i32) as u32 % self.width;
            robot.y = (robot.y as i32 + robot.v_y + self.height as i32) as u32 % self.height;
        }
    }

    fn safety_factor(&mut self) -> u32 {
        let omitted_x = self.width / 2;
        let omitted_y = self.height / 2;
        let mut area_a = 0;
        let mut area_b = 0;
        let mut area_c = 0;
        let mut area_d = 0;
        for robot in &self.robots {
            if robot.x < omitted_x && robot.y < omitted_y {
                area_a += 1;
            } else if robot.x > omitted_x && robot.y < omitted_y {
                area_b += 1;
            } else if robot.x < omitted_x && robot.y > omitted_y {
                area_c += 1;
            } else if robot.x > omitted_x && robot.y > omitted_y {
                area_d += 1;
            }
        }
        return area_a * area_b * area_c * area_d;
    }

    fn density(&self) -> u32 {
        let mut grid: Vec<Vec<bool>> = vec![vec![false; self.width as usize]; self.height as usize];
        for robot in &self.robots {
            grid[robot.y as usize][robot.x as usize] = true;
        }
        return grid.iter().map(|row| row.iter().filter(|&&cell| cell).count()).fold(0, |acc, x| acc + x as u32);
    }
}

fn main() {
    let filename = std::env::args().nth(1).expect("No filename provided");
    let width = std::env::args().nth(2).expect("No width provided").parse::<u32>().unwrap();
    let height = std::env::args().nth(3).expect("No height provided").parse::<u32>().unwrap();
    let robots: Vec<Robot> = std::fs::read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .map(|line| Robot::from(line))
        .collect();
    let mut room = Room::new(width, height, robots);
    let mut clone = room.clone();
    println!("{}", room);
    let mut density = 0;
    for i in 0..width*height {
        room.tick();
        let new_density = room.density();
        if new_density > density {
            density = new_density;
            println!("{}:\n{}", i,room);
        }
    }
    println!("Safety factor: {}", room.safety_factor());
    let mut i = 0;
    while clone.density() != density {
        clone.tick();
        i += 1;
    }
    println!("{}", clone);
    println!("Seconds: {}", i);
}
