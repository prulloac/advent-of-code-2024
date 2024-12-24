use std::{collections::HashMap, fmt::{Debug, Display, Formatter, Result}};

#[derive(Eq, PartialEq)]
#[derive(Hash)]
#[derive(Clone, Copy)]
struct Pair {
    val: usize,
    depth: usize,
}

impl Pair {
    fn new(val: usize, depth: usize) -> Self {
        Self { val, depth }
    }
}

#[derive(Eq, PartialEq)]
#[derive(Hash)]
#[derive(Clone)]
struct Rock {
    value: String,
    int_val: usize,
}

impl Display for Rock {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.value)
    }
}

impl Debug for Rock {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.value)
    }
}

impl Rock {
    fn new(int_val:usize) -> Self {
        let value = int_val.to_string();
        Self { value, int_val }
    }
}

fn main() {
    let filename = std::env::args().nth(1).expect("Filename required");
    let blinks: usize = std::env::args().nth(2).expect("Blinks required").parse().unwrap();
    let rocks: Vec<Rock> = std::fs::read_to_string(filename)
        .expect("Can't read file")
        .lines()
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .flatten()
        .map(|rock| Rock::new(rock.to_string().parse().unwrap()))
        .collect::<Vec<Rock>>();

    println!("Initial arrangement: ");
    println!("{:?}", rocks);
    let mut cache = HashMap::new();
    let mut total = 0;
    for rock in rocks.iter() {
        total += recursive_blink(rock.clone(), blinks, &mut cache);
    }
    println!("Total: {}", total);
}

fn recursive_blink(rock: Rock, depth: usize, cache: &mut HashMap<Pair, usize>) -> usize {
    let key = Pair::new(rock.int_val, depth);
    // if element is in cache, return it
    if cache.get(&key).is_some() {
        return cache.get(&key).unwrap().clone();
    }
    // if depth is 0, return 1
    if depth == 0 {
        return 1;
    }
    // else, calculate the value and insert it into cache
    // if value is 0, change it to 1 and recursively call the function with depth-1
    if rock.int_val == 0 {
        let nrock = Rock::new(1);
        let val = recursive_blink(nrock, depth-1, cache);
        cache.insert(key, val);
    // if value length is even, split it into two rocks and recursively call the function
    } else if rock.value.len() % 2 == 0 {
        let left_val = rock.value.chars().take(rock.value.len() / 2).collect::<String>();
        let left_rock = Rock::new(left_val.parse().unwrap());
        let left_result = recursive_blink(left_rock, depth-1, cache);
        let right_val = rock.value.chars().skip(rock.value.len() / 2).collect::<String>();
        let right_rock = Rock::new(right_val.parse().unwrap());
        let right_result = recursive_blink(right_rock, depth-1, cache);
        cache.insert(key, left_result + right_result);
    } else {
        let nrock = Rock::new(rock.int_val*2024);
        let val = recursive_blink(nrock, depth-1, cache);
        cache.insert(key, val);
    }
    return cache.get(&key).unwrap().clone();
}