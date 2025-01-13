use std::fs::read_to_string;

struct Rule {
    left: u32,
    right: u32,
}

impl Rule {
    fn new(left: u32, right: u32) -> Rule {
        Rule { left, right }
    }

    fn print(&self) {
        println!("Rule: {} -> {}", self.left, self.right);
    }
}

struct Update {
    values: Vec<u32>,
}

impl Update {
    fn new(values: Vec<u32>) -> Update {
        Update { values }
    }

    fn print(&self) {
        for value in self.values.iter() {
            print!("{}, ", value);
        }
        println!();
    }

    fn iter(&self) -> std::slice::Iter<u32> {
        self.values.iter()
    }

    fn swap(&mut self, left: usize, right: usize) {
        let temp = self.values[left];
        self.values[left] = self.values[right];
        self.values[right] = temp;
    }

    fn contains(&self, value: &u32) -> bool {
        self.values.contains(value)
    }

    fn is_valid(&self, rules: &Vec<Rule>) -> bool {
        let mut valid = true;
        for rule in rules.iter() {
            let left = rule.left;
            let right = rule.right;
            // check if neither left nor right are in the values
            // if so, the update is still valid and we can continue
            if !self.values.contains(&left) || !self.values.contains(&right) {
                continue;
            }
            // if we reach this point, we have to check if the values are in the correct order
            // if they are not, the update is invalid
            let left_index = self.iter().position(|&x| x == left).unwrap();
            let right_index = self.iter().position(|&x| x == right).unwrap();
            if left_index > right_index {
                valid = false;
                break;
            }
        }
        valid
    }

    fn get_middle(&self) -> u32 {
        let len = self.values.len();
        let middle = len / 2;
        self.values[middle]
    }

    fn fix_with_rules(&self, rules: &Vec<Rule>) -> Update {
        let mut values = Update::new(self.values.clone());
        // first we filter out the rules that do not contain any of the values
        let mut new_rules: Vec<Rule> = Vec::new();
        rules.iter().for_each(|rule| {
            let left = rule.left;
            let right = rule.right;
            if values.contains(&left) || values.contains(&right) {
                let r = Rule::new(left, right);
                new_rules.push(r);
            }
        });
        // now we iterate until the update is valid
        while !values.is_valid(&new_rules) {
            // if we reach this point, we have to swap some values
            // we iterate over the rules and check if the values are in the correct order
            for rule in new_rules.iter() {
//                println!("Checking rule: {} -> {}... ", rule.left, rule.right);
                let left = rule.left;
                let right = rule.right;
                // if we reach this point, we have to check if the values are in the correct order
                // if they are not, we have to swap them
                let left_index_option = values.iter().position(|&x| x == left);
                let left_index = match left_index_option {
                    None => continue,
                    Some(index) => index,
                    
                };
                let right_index_option = values.iter().position(|&x| x == right);
                let right_index = match right_index_option {
                    None => continue,
                    Some(index) => index,
                };
                if left_index > right_index {
//                    print!("Swapping {} and {}... ", left, right);
                    values.swap(left_index, right_index);
                } else {
//                    println!("Values are in correct order: {}", values.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "));
                }
            }

        }
        Update::new(values.values.clone())
    }
}

struct Input {
    rules: Vec<Rule>,
    updates: Vec<Update>,
}

fn main() {
    let input = std::env::args().nth(1).expect("Please provide an input");

    let sleigh_launch_safety_manual = parse_input(&input);

    sleigh_launch_safety_manual.rules.iter().for_each(|rule| rule.print());
    sleigh_launch_safety_manual.updates.iter().for_each(|update| update.print());

    println!("Valid updates:");

    let mut middle_sum = 0;
    let mut invalid_updates = Vec::new();
    for update in sleigh_launch_safety_manual.updates.iter() {
        if update.is_valid(&sleigh_launch_safety_manual.rules) {
            update.print();
            middle_sum += update.get_middle();
        } else {
            invalid_updates.push(update);
        }
    }

    println!("Sum of middle values, before fix: {}", middle_sum);

    // fix the invalid updates
    middle_sum = 0;
    println!("Invalid updates, now fixed:");
    for update in invalid_updates.iter() {
        let fixed_update: Update = update.fix_with_rules(&sleigh_launch_safety_manual.rules);
        fixed_update.print();
        middle_sum += fixed_update.get_middle();
    }

    println!("Sum of middle values, only fixed updates: {}", middle_sum);
    

}

fn parse_input(filename: &str) -> Input {
    let mut rules: Vec<Rule> = Vec::new();
    let mut updates: Vec<Update> = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        // if line contains a bar | then we have a rule
        // otherwise we have an update
        if line.contains("|") {
            let mut parts = line.split("|");
            let left = parts.next().unwrap().parse().unwrap();
            let right = parts.next().unwrap().parse().unwrap();
            rules.push(Rule::new(left, right));
        } else if line.contains(",") {
            // updates are integers separated by commas
            // we need to split the line by commas
            // and parse each integer
            let update = Update::new(line.split(",").map(|x| x.parse().unwrap()).collect());
            updates.push(update);
        }
    }
    Input { rules, updates }
}