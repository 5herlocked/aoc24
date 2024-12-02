use std::{fs::read_to_string, result};

fn parse_and_validate(filename: &str) -> Vec<Vec<u8>> {
    // Read and parse the file
    let contents = read_to_string(filename).expect("Failed to read file");
    
    // Parse each line into a vector of numbers
    let number_sequences: Vec<Vec<u8>> = contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u8>().unwrap())
                .collect()
        })
        .collect();

    number_sequences
}

fn all_sorted(numbers: &Vec<u8>, size: usize) -> bool {
    numbers.windows(size).all(|w| w.is_sorted_by(|a, b| a < b)) || 
    numbers.windows(size).all(|w| w.is_sorted_by(|a, b| a > b))
}

fn is_within_relative_range(numbers: &Vec<u8>) -> bool {
    numbers.windows(2).all(|w| {
        let diff = w[0].abs_diff(w[1]);
        diff >= 1 && diff <= 3
    })
}

fn main() {
    let results = parse_and_validate("two.input.txt");
    let mut valid_reports = 0;

    let mut damped_reports = 0;

    results.iter().for_each(|result| {
        let is_valid: bool = all_sorted(result,  2) && is_within_relative_range(result);
        if is_valid {
            valid_reports += 1;
            return;
        }

        // Brute force through all failed lists to see if dropping ANY one value makes a difference in the outcome
        // Smart way would be to return from the failure state (in what window we fail), and then just try dropping that
        // and then fall back to this
        for i in 0..result.len() {
            let mut damped_result = result.clone();

            damped_result.remove(i);

            let is_valid: bool = all_sorted(&damped_result, 2) && is_within_relative_range(&damped_result);
            if is_valid {
                damped_reports += 1;
                return;
            }
        }
    });

    println!("Valid reports: {}, Damped reporst: {}, Passed Reports: {}",
     valid_reports,  damped_reports, valid_reports + damped_reports);
}
