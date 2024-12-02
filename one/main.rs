use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::ops::Mul;

fn parse_file_to_lists(path: &str) -> io::Result<(Vec<u32>, Vec<u32>)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    
    let mut first_numbers = Vec::new();
    let mut second_numbers = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut numbers = line.split_whitespace();
        
        if let (Some(first), Some(second)) = (numbers.next(), numbers.next()) {
            if let (Ok(num1), Ok(num2)) = (first.parse::<u32>(), second.parse::<u32>()) {
                first_numbers.push(num1);
                second_numbers.push(num2);
            }
        }
    }

    Ok((first_numbers, second_numbers))
}

fn list_distance() -> io::Result<()> {
    // Example 1: Collecting into two vectors
    let (mut list1, mut list2) = parse_file_to_lists("one.input.txt")?;
    println!("First list size: {:?}", list1.len());
    println!("Second list size: {:?}", list2.len());

    assert!(list1.len() == list2.len());

    // probably the most expensive part of this, can be paralelized.
    list1.sort();
    list2.sort();

    let mut abs_diffs = Vec::new();
    

    list1.iter().zip(list2.iter()).for_each(|(a, b)| {
        let diff = a.abs_diff(b.clone());
        abs_diffs.push(diff);
    });

    println!("Answer 1: {}", abs_diffs.iter().sum::<u32>());
    Ok(())
}

fn aggregator(list: &Vec<u32>) -> HashMap<u32, u32> {
    let mut result = HashMap::new();
    
    for num in list {
        *result.entry(num.clone()).or_insert(0) += 1;
    }
    
    result
}


fn simmilarity_list() -> io::Result<()> {
    // Example 1: Collecting into two vectors
    let (list1, list2) = parse_file_to_lists("one.input.txt")?;
    println!("First list size: {:?}", list1.len());
    println!("Second list size: {:?}", list2.len());

    assert!(list1.len() == list2.len());

    let aggregate_two = aggregator(&list2);
    let mut res = Vec::new();

    list1.iter().for_each(| a | {
        let similarity_sub_score: u32 = a * aggregate_two.get(a).cloned().unwrap_or_default();
        res.push(similarity_sub_score);
    });

    println!("Similarity Score: {}", res.iter().sum::<u32>());
    Ok(())
}

fn main() -> io::Result<()> {
    // list_distance()

    simmilarity_list()
}
