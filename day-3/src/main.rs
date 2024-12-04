use std::fs::read_to_string;
use regex::Regex;

struct Multiplication {
    factor_a: i32,
    factor_b: i32,
}

fn main() {
    let input = std::env::args().nth(1).expect("Please provide an input");
    let multiplications = parse_input(&input);

    let mut sum = 0;
    for multiplication in &multiplications {
        println!("Multiplication: {} * {}", multiplication.factor_a, multiplication.factor_b);
        sum += multiplication.factor_a * multiplication.factor_b;
    }

    println!("Sum: {}", sum);

}

fn parse_input(filename: &str) -> Vec<Multiplication> {
    let memory = read_to_string(filename).unwrap();
    // traverse through the string "memory" by characters
    // and use a flag to determine if we are inside the expected pattern of mul(a,b) or not
    // if we are inside the pattern, we will collect the numbers and append them to a vector of Multiplication
    // if we are not inside the pattern, we will ignore the characters and continue
    // finally, we will return the vector of Multiplication
    let mut output: Vec<Multiplication> = Vec::new();
    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\))").unwrap();
    let re2 = Regex::new(r"\d{1,3}").unwrap();
    let matches = re.find_iter(&memory).map(|m| m.as_str()).collect::<Vec<&str>>();
    let mut counts = true;
    for m in matches {
        println!("{}", m);
        if m == "don't()" {
            counts = false;
            continue;
        } else if m == "do()" {
            counts = true;
            continue;
        } else if counts {
            let mut numbers = re2.find_iter(m).map(|m| m.as_str()).collect::<Vec<&str>>();
            println!("{:?}", numbers);
            let factor_b = numbers.pop().unwrap().parse().unwrap();
            let factor_a = numbers.pop().unwrap().parse().unwrap();
            output.push(Multiplication { factor_a, factor_b });    
        }
    }
    output
}
