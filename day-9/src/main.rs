use std::{env::args, fs::read_to_string, u32};

struct Buffer {
    disk_map: Vec<u32>,
    unsolved: Vec<i32>,
    solved: Vec<i32>,
    file_ids: Vec<i32>,
    checksum: u128
}

fn hide_free(c: &i32) -> String {
    if *c == -1 {
        return ".".to_string();
    }
    (*c).to_string()
}

impl Buffer {
    fn new(input: String) -> Self {
        let unsolved = Vec::new();
        let solved = Vec::new();
        let file_ids = Vec::new();
        let checksum = 0;
        let disk_map = input.chars().map(|c| {
            match c {
                '0'..='9' => c.to_digit(10).unwrap(),
                _ => 0
            }
        }).collect::<Vec<u32>>();
        Self {
            disk_map,
            unsolved,
            solved,
            checksum,
            file_ids
        }
    }

    fn print(&self) {
        println!("Disk map: {}", self.disk_map.iter().map(|c| c.to_string()).collect::<String>());
        self.print_unsolved();
        self.print_solved();
        println!("Checksum: {}", self.checksum);
    }

    fn print_solved(&self) {
        println!("Solved: {}", self.solved.iter().map(hide_free).collect::<String>());
    }

    fn print_unsolved(&self) {
        println!("Unsolved: {}", self.unsolved.iter().map(hide_free).collect::<String>());
    }

    fn expand(&mut self) {
        let mut counter = 0;
        for x in self.disk_map.iter() {
            for _ in 0..*x {
                if counter % 2 == 1 {
                    self.unsolved.push(-1);
                } else {
                    let file_id = counter / 2;
                    self.unsolved.push(file_id);
                    if !self.file_ids.contains(&file_id) {
                        self.file_ids.insert(0,file_id); // this will keep the file ids in decreasing order
                    }
                }
            }
            counter += 1;
        }
    }

    fn reset_solved(&mut self) {
        self.solved.clear();
    }

    fn fit_algo_a(&mut self) {
        self.reset_solved();
        let mut idx_start = 0;
        let mut idx_end = self.unsolved.len() - 1;
        while idx_start <= idx_end {
            if self.unsolved[idx_start] != -1 {
                self.solved.push(self.unsolved[idx_start]);
                idx_start += 1;
            } else {
                if self.unsolved[idx_end] != -1 {
                    self.solved.push(self.unsolved[idx_end]);
                    idx_start += 1;
                    idx_end -= 1;
                } else {
                    idx_end -= 1;
                }
            }
        }
    }

    fn fit_algo_b(&mut self) {
        self.reset_solved();
        self.solved = self.unsolved.clone();
        let calc_block_size_from_idx = |v: &Vec<i32>, idx: usize| {
            let mut block_size = 0;
            let c = v[idx];
            for i in idx..v.len() {
                if v[idx+(i-idx)] == c {
                    block_size += 1;
                } else {
                    break;
                }
            }
            block_size
        };
        let index_of_leftmost_free_space_of_size_at_least = |v: &Vec<i32>, size: usize| {
            let mut return_idx: i32 = -1;
            let mut free_size = 0;
            for (idx, element) in v.iter().enumerate() {
                if free_size >= size {
                    return_idx = (idx - size) as i32;
                    break;
                }
                if *element == -1 {
                    free_size += 1;
                } else {
                    free_size = 0;
                }
            }
            return_idx
        };

        for i in self.file_ids.iter() {
            println!("Processing file id {}", i);
            // we calculate the size of the memory block that we want to store
            let file_idx = self.solved.iter().position(|&x| x == *i).unwrap();
            let file_size = calc_block_size_from_idx(&self.solved, file_idx);
            let free_available_idx = index_of_leftmost_free_space_of_size_at_least(&self.solved, file_size);
            //println!("File id {} has size {}, leftmost available free memory at idx {}", *i, file_size, free_available_idx);
            // if we reach the end of the memory, reduce the idx_end by the block size
            if free_available_idx == -1 || file_idx < free_available_idx as usize {
                continue;
            }
            //println!("Moving file id {} from position {} to free memory at {}", *i, file_idx, free_available_idx);
            // if block size is smaller or equal than free size
            for k in 0..file_size {
                //we store the block in the free memory
                self.solved[free_available_idx as usize + k] = *i;
                // we remove the block from the end
                self.solved[file_idx + k] = -1;
            }
            //self.print_solved();
        }
    }

    fn calculate_checksum(&mut self) {
        let mut checksum: u128 = 0;
        for (idx, x) in self.solved.iter().enumerate() {
            if *x == -1 {
                continue;
            }
            checksum += *x as u128 * (idx as u128);
        }
        self.checksum = checksum as u128;
    }
}

fn main() {
    let filename = args().nth(1).expect("Please provide an input");
    let mut buffer = Buffer::new(read_to_string(filename).expect("Failed to read the file"));
    buffer.expand();
    println!("Part 1");
    buffer.print_unsolved();
    buffer.fit_algo_a();
    buffer.calculate_checksum();
    buffer.print();
    println!("\n\n\n\n");
    println!("Part 2");
    buffer.print_unsolved();
    buffer.fit_algo_b();
    buffer.calculate_checksum();
    buffer.print();
}
