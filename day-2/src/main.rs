use std::fs::read_to_string;

struct Report {
    levels: Vec<i32>,
}

enum Safety {
    Safe,
    Unsafe,
}

fn main() {
    let input = std::env::args().nth(1).expect("Please provide an input");
    let reports = parse_input(&input);

    let mut counter = 0;
    for report in &reports {
        for level in &report.levels {
            print!("{} ", level);
        }
        match check_safety(report.levels.clone()) {
            Safety::Safe => {
                print!("Safe");
                counter += 1;
            },
            Safety::Unsafe => print!("Unsafe"),
        }
        print!("\n");
    }
    print!("Safe reports: {}\n", counter);
    println!("----------------");

    counter = 0;
    for report in &reports {
        for level in &report.levels {
            print!("{} ", level);
        }
        match check_safety_loosely(report.levels.clone()) {
            Safety::Safe => {
                print!("Safe");
                counter += 1;
            },
            Safety::Unsafe => print!("Unsafe"),
        }
        print!("\n");
    }
    print!("Safe reports: {}\n", counter);
}

fn parse_input(filename: &str) -> Vec<Report> {
    let mut output = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        let iter = line.split_whitespace();
        let mut levels = Vec::new();
        for item in iter {
            levels.push(item.parse().unwrap());
        }
        output.push(Report { levels });
    }

    output
}

fn check_safety(levels: Vec<i32>) -> Safety {
    // first check if strict asc or desc
    if !check_asc(levels.clone()) && !check_desc(levels.clone()) {
        Safety::Unsafe
    } else {
        Safety::Safe
    }
}

fn check_asc(levels: Vec<i32>) -> bool {
    // check if level is in strict ascending order, and difference between levels is between 1 and 3
    let mut is_ascending = true;
    for i in 0..levels.len() - 1 {
        let left = levels[i];
        let right = levels[i + 1];
        if right - left < 1 || right - left > 3 {
            is_ascending = false;
            break;
        }
    }
    is_ascending
}

fn check_desc(levels: Vec<i32>) -> bool {
    // check if level is in strict descending order, and difference between levels is between 1 and 3
    let mut is_descending = true;
    for i in 0..levels.len() - 1 {
        let left = levels[i];
        let right = levels[i + 1];
        if left - right < 1 || left - right > 3 {
            is_descending = false;
            break;
        }
    }
    is_descending
}

fn check_safety_loosely(levels: Vec<i32>) -> Safety {
    // first check if asc or desc
    if !check_asc_loosely(levels.clone()) && !check_desc_loosely(levels.clone()) {
        Safety::Unsafe
    } else {
        Safety::Safe
    }
}

fn check_asc_loosely(levels: Vec<i32>) -> bool {
    // check if level is in ascending order, and difference between levels is between 1 and 3
    // the loose version of check allows for "what if we remove a single element from the list"
    // so we raw dog it and attempt to remove every element one time and check if the list is still in compliance
    if check_asc(levels.clone()) {
        return true;
    }
    for i in 0..levels.len() {
        let mut temp = levels.clone();
        temp.remove(i);
        if check_asc(temp) {
            return true;
        }
    }
    false
}

fn check_desc_loosely(levels: Vec<i32>) -> bool {
    // check if level is in descending order, and difference between levels is between 1 and 3
    // the loose version of check allows for "what if we remove a single element from the list"
    // so we raw dog it and attempt to remove every element one time and check if the list is still in compliance
    if check_desc(levels.clone()) {
        return true;
    }
    for i in 0..levels.len() {
        let mut temp = levels.clone();
        temp.remove(i);
        if check_desc(temp) {
            return true;
        }
    }
    false
}