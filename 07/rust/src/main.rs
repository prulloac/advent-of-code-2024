use std::fs::read_to_string;

struct Calibration {
    result: i64,
    input: Vec<i64>
}

impl Calibration {
    fn new(result: i64, input: Vec<i64>) -> Calibration {
        Calibration {
            result,
            input
        }
    }

    fn parse(line: &str) -> Calibration {
        let mut parts = line.split(": ");
        let result: i64 = parts.next().unwrap().parse().unwrap();
        let input: Vec<i64> = parts.next().unwrap().split(" ").map(|x| x.parse().unwrap()).collect();
        Calibration::new(result, input)
    }

    fn print(&self) {
        println!("{}: {:?}", self.result, self.input);
    }

    fn recursive_is_valid(acc: i64, idx: i32, input: &[i64], result: i64) -> bool {
        if idx as usize == input.len() {
            return acc == result;
        }
        return Calibration::recursive_is_valid(acc + input[idx as usize], idx + 1, input, result) ||
            Calibration::recursive_is_valid(acc * input[idx as usize], idx + 1, input, result);
    }

    fn recursive_is_valid_with_concat(acc: i64, idx: i32, input: &[i64], result: i64) -> bool {
        if idx as usize == input.len() {
            return acc == result;
        }
        let concatenated = |a: i64, b: i64| -> i64 { 
            let a_str = a.to_string();
            let b_str = b.to_string();
            format!("{}{}", a_str, b_str).parse().unwrap() 
        };
        return Calibration::recursive_is_valid_with_concat(acc + input[idx as usize], idx + 1, input, result) ||
            Calibration::recursive_is_valid_with_concat(acc * input[idx as usize], idx + 1, input, result) ||
            Calibration::recursive_is_valid_with_concat(concatenated(acc,input[idx as usize]), idx + 1, input, result);
    }

    fn is_valid(&self) -> bool {
        Calibration::recursive_is_valid(self.input[0], 1, &self.input, self.result as i64)
    }

    fn is_valid_with_concat(&self) -> bool {
        Calibration::recursive_is_valid_with_concat(self.input[0], 1, &self.input, self.result as i64)
    }

}

fn main() {
    let input = std::env::args().nth(1).expect("Please provide an input");

    let calibrations = parse_input(&input);

    calibrations.iter().for_each(|calibration| {
        calibration.print();
    });

    let mut filtered = calibrations.iter()
        .filter(|calibration| (*calibration).is_valid())
        .collect::<Vec<_>>();

    println!("Valid calibrations:");
    filtered.iter().for_each(|calibration| calibration.print());

    filtered.iter()
        .map(|c| c.result)
        .reduce(|a, b| a + b)
        .map(|sum| println!("Sum: {}", sum));

    filtered = calibrations.iter()
        .filter(|calibration| (*calibration).is_valid_with_concat())
        .collect::<Vec<_>>();

    println!("Valid calibrations with concatenation:");
    filtered.iter().for_each(|calibration| calibration.print());

    filtered.iter()
        .map(|c| c.result)
        .reduce(|a, b| a + b)
        .map(|sum| println!("Sum: {}", sum));
}

fn parse_input(filename: &str) -> Vec<Calibration> {
    read_to_string(filename).unwrap().lines().map(|line| Calibration::parse(line)).collect()
}