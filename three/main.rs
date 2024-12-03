use regex::{Regex, RegexSet};
use std::{
    fs::read_to_string,
    io::{self, BufReader, Read},
};

fn input_tokenizer(file_path: &str) -> Vec<&str> {
    // greedy parser, longest string first

    let input = std::fs::File::open(file_path).unwrap();

    let reader = BufReader::new(input);

    loop {}
}

// regex solution
fn input_tokenizer_regex(file_path: &str) -> Vec<(u32, u32)> {
    let input = read_to_string(file_path).expect("Failed to open and read file");

    let first_mults = Regex::new(r"(?P<first>.*)don't\(\)").unwrap();

    let all_do = Regex::new(r"do\(\)(?P<enabled>.*)(don't\(\))?").unwrap();

    let re = Regex::new(r"mul\((?P<lhs>\d*),(?P<rhs>\d*)\)").unwrap();

    let mut tokenized = first_mults
            .find(&input)
            .map(|found| {
                let first_ops = found.as_str();

                // enabled ops could have more than one muls in them so, this will return Vec<(u32, u32)>
                re.captures_iter(first_ops)
                    .map(|cap| {
                        let lhs = cap.name("lhs").unwrap().as_str().parse::<u32>().unwrap();
                        let rhs = cap.name("rhs").unwrap().as_str().parse::<u32>().unwrap();

                        (lhs, rhs)
                    })
                    .collect::<Vec<(u32, u32)>>()
            })
            .unwrap_or(vec![]);

    println!("Before first don't: {:?}\n", &tokenized);

    tokenized.extend(
        all_do
            .find_iter(&input)
            .map(|enabled_inner| {
                // enabled ops could have more than one muls in them so, this will return Vec<(u32, u32)>
                re.captures_iter(enabled_inner.as_str())
                    .map(|cap| {
                        let lhs = cap.name("lhs").unwrap().as_str().parse::<u32>().unwrap();
                        let rhs = cap.name("rhs").unwrap().as_str().parse::<u32>().unwrap();

                        (lhs, rhs)
                    })
                    .collect::<Vec<(u32, u32)>>()
            })
            .flatten()
            .into_iter(),
    );

    println!("Final List: {:?}\n", &tokenized);

    tokenized
}

fn input_tester(input: &str) -> Vec<(u32, u32)> {
    let first_mults = Regex::new(r"^(?P<first>.*)don't\(\)").unwrap();

    let all_do = Regex::new(r"do\(\)(?P<enabled>.*)(don't\(\))?").unwrap();

    let re = Regex::new(r"mul\((?P<lhs>\d*),(?P<rhs>\d*)\)").unwrap();

    let mut tokenized = first_mults
        .find(input)
        .map(|found| {
            let first_ops = found.as_str();

            // enabled ops could have more than one muls in them so, this will return Vec<(u32, u32)>
            re.captures_iter(first_ops)
                .map(|cap| {
                    let lhs = cap.name("lhs").unwrap().as_str().parse::<u32>().unwrap();
                    let rhs = cap.name("rhs").unwrap().as_str().parse::<u32>().unwrap();

                    (lhs, rhs)
                })
                .collect::<Vec<(u32, u32)>>()
        })
        .unwrap_or(vec![]);

    println!("{:?}", &tokenized);

    tokenized.extend(
        all_do
            .captures_iter(&input)
            .map(|enabled_inner| {
                let enabled_ops = enabled_inner.name("enabled").unwrap().as_str();

                // enabled ops could have more than one muls in them so, this will return Vec<(u32, u32)>
                re.captures_iter(enabled_ops)
                    .map(|cap| {
                        let lhs = cap.name("lhs").unwrap().as_str().parse::<u32>().unwrap();
                        let rhs = cap.name("rhs").unwrap().as_str().parse::<u32>().unwrap();

                        (lhs, rhs)
                    })
                    .collect::<Vec<(u32, u32)>>()
            })
            .flatten()
            .into_iter(),
    );

    println!("{:?}", &tokenized);

    tokenized
}

fn main() -> io::Result<()> {
    let nums = input_tokenizer_regex("./three.input.txt");

    // let nums = input_tester("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");

    let val: u32 = nums.iter().map(|(lhs, rhs)| -> u32 { lhs * rhs }).sum();

    println!("Sum of all mults: {}", val);

    Ok(())
}
