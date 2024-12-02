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

fn all_sorted(numbers: &Vec<u8>, size: usize) -> Result<(), usize> {
    // Check ascending order
    let ascending_fail = numbers.windows(size)
        .enumerate()
        .find(|(_, w)| !w.is_sorted_by(|a, b| a < b))
        .map(|(i, _)| i);
    
    // Check descending order
    let descending_fail = numbers.windows(size)
        .enumerate()
        .find(|(_, w)| !w.is_sorted_by(|a, b| a > b))
        .map(|(i, _)| i);
    
    // If both checks failed, return the first failing index
    match (ascending_fail, descending_fail) {
        (Some(asc_idx), Some(desc_idx)) => Err(asc_idx.min(desc_idx)),
        _ => Ok(())
    }
}

fn is_within_relative_range(numbers: &Vec<u8>) -> Result<(), usize> {
    match numbers.windows(2)
        .enumerate()
        .find(|(_, w)| {
            let diff = w[0].abs_diff(w[1]);
            diff < 1 || diff > 3
        }) {
        Some((idx, _)) => Err(idx),
        None => Ok(())
    }
}

fn drop_and_test(input: Vec<u8>, to_drop: usize) -> bool {
    let mut test_input = input.clone();
    test_input.remove(to_drop);
    match (all_sorted(&test_input, 2), is_within_relative_range(&test_input)) {
        (Ok(_), Ok(_)) => true,
        _ => false,
    }
}

fn main() {
    let results = parse_and_validate("two.input.txt");
    let mut valid_reports = 0;

    let mut damped_slow = 0;

    let mut damped_fast = 0;

    results.iter().for_each(|result| {
        match (all_sorted(&result, 2), is_within_relative_range(&result)) {
            (Ok(_), Ok(_)) => { valid_reports += 1; return; },  // All pass
            (Ok(_), Err(e)) => {  // relative range failed
                // Try to drop the value at the error index
                if drop_and_test(result.clone(), e + 1) {
                    damped_fast += 1;
                    return;
                }  
            },
            (Err(e), Ok(_)) => { // Sorted failed
                if drop_and_test(result.clone(), e + 1) {
                    damped_fast += 1;
                    return;
                }
            },  // not sorted
            (Err(sort_e), Err(range_e)) => {  // not sorted and some range error
                if sort_e == range_e {
                    // Both failed at the same index
                    if drop_and_test(result.clone(), sort_e + 1) {
                        damped_fast += 1;
                        return;
                    }
                } else {
                    let mut damped_result = result.clone();
                    damped_result.remove(sort_e + 1);
                    match (all_sorted(&damped_result, 2), is_within_relative_range(&damped_result)) {
                        (Ok(_), Ok(_)) => {damped_fast += 1; return;},  // All pass
                        _ => {
                            damped_result = result.clone();
                            damped_result.remove(range_e + 1);
                            match (all_sorted(&damped_result, 2), is_within_relative_range(&damped_result)) {
                                (Ok(_), Ok(_)) => {damped_fast += 1; return;},  // All pass
                                _ => {
                                    // if it doesn't work, we don't try to be cute and just brute force it
                                },
                            }
                        },
                    }
                }
            }, 
        }

        // Brute force through all failed lists to see if dropping ANY one value makes a difference in the outcome
        // Smart way would be to return from the failure state (in what window we fail), and then just try dropping that
        // and then fall back to this
        for i in 0..result.len() {
            let mut damped_result = result.clone();

            damped_result.remove(i);

            match (all_sorted(&damped_result, 2), is_within_relative_range(&damped_result)) {
                (Ok(_), Ok(_)) => { damped_slow += 1; return; },  // All pass and we only made one change
                _ => {
                    // if it doesn't work, we continue
                },
            }
        }
    });

    println!("Valid reports: {}, Damped Slow: {}, Damped Fast: {}, Passed Reports: {}",
     valid_reports,  damped_slow, damped_fast, valid_reports + damped_fast + damped_slow);
}
