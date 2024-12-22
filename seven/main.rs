use std::{collections::HashMap, fs::read_to_string};


fn parse_line(input: &str) -> (usize, Vec<u32>) {
    let mut parts = input.split(": ");
    let sum = parts.next().unwrap().parse::<usize>().unwrap();
    let nums = parts.next().unwrap().split(" ").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    (sum, nums)
}

fn parse_input(input: &str) -> HashMap<usize, Vec<u32>> {
    let mut map = HashMap::new();
    for line in input.lines() {
        let (sum, nums) = parse_line(line);
        map.insert(sum, nums);
    }
    map
}

fn kick_off(val: usize, nums: Vec<u32>) -> (usize, bool) {
    let first = nums[0] as usize;
    let recurse_vec = nums[1..].to_vec();

    let add_val = recursive(val, first, &recurse_vec, false);
    let mul_val = recursive(val, first, &recurse_vec, true);

    (val, add_val || mul_val)
}

fn recursive(goal: usize, curr_val: usize, next: &Vec<u32>, multiply: bool) -> bool {
    if curr_val == goal {
        return true;
    } else if curr_val > goal {
        return false;
    } else if next.len() == 0 {
        return false;
    } else {
        let new_val = if multiply {curr_val * (next[0] as usize)} else {curr_val + (next[0] as usize)};

        let inner_next = next[1..].to_vec();

        let mul_val = recursive(goal, new_val, &inner_next, true);
        let add_val = recursive(goal, new_val, &inner_next, false);

        return mul_val || add_val 
    }
}

fn main() {
    let input = parse_input(read_to_string("seven.input.txt").unwrap().as_str());

    let res: usize = input.iter()
            .map(|(sum, nums)| kick_off(*sum, nums.clone()))
            .filter_map(|(val, valid)| if valid {Some(val)} else {None})
            .sum();

    println!("Sum of valid equations: {:?}", res);
}


#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";


    #[test]
    fn test_parse_input() {
        let input = parse_input(TEST_INPUT);

        assert!(input.len() == 9);
        assert!(input.contains_key(&(21037 as usize)));
    }
    
    #[test]
    fn evaluate_valid() {
        let mut input = parse_input(TEST_INPUT);

        let res: usize = input.iter_mut()
                .map(|(sum, nums)| kick_off(*sum, nums.clone()))
                .filter_map(|(val, valid)| if valid {Some(val)} else {None})
                .sum();

        println!("Sum of valid equations: {:?}", res);
    }
}