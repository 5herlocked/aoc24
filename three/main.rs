use regex::{Regex, RegexSet};
use std::{
    fs::read_to_string,
    io::{self, BufReader, Read},
};

fn input_tokenizer(file_path: &str) -> Vec<(u32, u32)> {
    // greedy parser, shortest string first

    let input = std::fs::File::open(file_path).unwrap();

    let reader = BufReader::new(input);

    loop {}
}

// regex solution
fn input_tokenizer_regex(file_path: &str) -> Vec<(u32, u32)> {
    let raw_input = read_to_string(file_path).expect("Failed to open and read file");
    let input = format!("do(){raw_input}");

    let re = Regex::new(r"mul\((?P<lhs>\d+),(?P<rhs>\d+)\)").unwrap();

    let mut nums: Vec<(u32, u32)> = Vec::new();

    for disabled in input.split("don't()") {
        for enabled in disabled.split("do()").skip(1) {
            for capture in re.captures_iter(enabled) {
                let lhs = capture.name("lhs").unwrap().as_str().parse::<u32>().unwrap();
                let rhs = capture.name("rhs").unwrap().as_str().parse::<u32>().unwrap();
                nums.push((lhs, rhs));
            }
        }

    }

    nums
}

fn main() -> io::Result<()> {
    let nums = input_tokenizer_regex("./three.input.txt");

    let val: u32 = nums.iter().map(|(lhs, rhs)| -> u32 { lhs * rhs }).sum();

    println!("Sum of all mults: {}", val);

    Ok(())
}
