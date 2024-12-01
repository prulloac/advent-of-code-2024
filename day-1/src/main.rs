use std::fs::read_to_string;

struct Lists {
    list_a: Vec<i32>,
    list_b: Vec<i32>,
}

fn main() {
    let input = std::env::args().nth(1).expect("Please provide an input");
    let lists = parse_input(&input);

    for (a, b) in lists.list_a.iter().zip(lists.list_b.iter()) {
        println!("{} {}", a, b);
    }
    print!("----------------\n");

    let mut sorted_a = sort_asc(&lists.list_a);
    let mut sorted_b = sort_asc(&lists.list_b);

    for (a, b) in sorted_a.iter().zip(sorted_b.iter()) {
        println!("{} {}", a, b);
    }
    print!("----------------\n");

    let mut distances = Vec::new();

    while !sorted_a.is_empty() && !sorted_b.is_empty() {
        let a = sorted_a.pop().unwrap();
        let b = sorted_b.pop().unwrap();

        // check if the difference is negative
        // if it is, we need to add the absolute value
        // of the difference to the distances list

        if (a - b) < 0 {
            distances.push(b - a);
        } else {
            distances.push(a - b);
        }
    }
    
    let mut sum = 0;
    for distance in distances {
        sum += distance;
        println!("{}", distance);
    }

    println!("Sum: {}\n", sum);
    print!("----------------\n");

    // similarity scores
    let mut scores = Vec::new();

    // similarity is calculated by how many times an item in list a appears in list b), multiplied by the item itself
    for a in lists.list_a.iter() {
        let mut count = 0;
        for b in lists.list_b.iter() {
            if a == b {
                count += 1;
            }
        }
        scores.push(count * a);
    }

    let mut similarity = 0;
    for score in scores {
        println!("{}", score);
        similarity += score;
    }
    print!("Similarity: {}\n", similarity);

}

fn parse_input(filename: &str) -> Lists {
    let mut output = Lists {
        list_a: Vec::new(),
        list_b: Vec::new(),
    };

    for line in read_to_string(filename).unwrap().lines() {
        let mut iter = line.split_whitespace();
        output.list_a.push(iter.next().unwrap().parse().unwrap());
        output.list_b.push(iter.next().unwrap().parse().unwrap());
    }

    output
}

// sort_asc sorts a list of integers in ascending order
fn sort_asc(list: &Vec<i32>) -> Vec<i32> {
    let mut sorted = list.clone();
    sorted.sort_unstable();
    sorted
}