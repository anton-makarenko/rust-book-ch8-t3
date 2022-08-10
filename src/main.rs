use std::io::stdin;
use std::collections::HashMap;

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let mut inputs: Vec<String> = input
            .split_whitespace()
            .map(|x| String::from(x))
            .collect();

        if inputs[0].eq("Exit") {
            break;
        }

        let department = inputs.remove(3);
        let employee = inputs.swap_remove(1);
        let employees = departments.entry(department).or_insert(Vec::new());
        (*employees).push(employee);
    }

    for (key, value) in &departments {
        println!("{}", key);
        for employee in value {
            println!("{}", *employee);
        }
        println!();
    }
}
