mod test;

use std::fs::read_to_string;

fn extract_caloric_sums(data: &str) -> Vec<usize> {
    let mut calories = data.split("\n\n").map(|chunk| {
        chunk.split("\n").map(|line| {
            line.parse::<usize>().unwrap_or(0)
        }).sum()
    }).collect::<Vec<usize>>();

    calories.sort();
    calories
}

fn sum_max_calories(data: &str, amount: usize) -> usize {
    let calories = extract_caloric_sums(data);
    calories[(calories.len() - amount)..calories.len()].iter().sum()
}

fn get_max_calories(data: &str) -> usize {
    sum_max_calories(data, 1)
}

fn main() {
    let data = read_to_string("final-data.txt").expect("Error reading file");

    println!("PART 1: {}", get_max_calories(data.as_str()));
    println!("PART 2: {}", sum_max_calories(data.as_str(), 3));
}
