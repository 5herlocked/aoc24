use std::collections::HashMap;

#[derive(Default)]
struct PageOrderingConstraints {
    before: Vec<isize>,  // Pages this page is dependent on
    after: Vec<isize>,  // Pages that depend on this page
}

fn parse_rules(input: &Vec<(isize, isize)>) -> HashMap<isize, PageOrderingConstraints> {
    let mut rules: HashMap<isize, PageOrderingConstraints> = HashMap::new();

    for (constraint, page) in input.clone() {
        rules.entry(page).or_default().before.push(constraint);

        rules.entry(constraint).or_default().after.push(page);
    }

    rules
}

fn split_input(input: &str) -> (Vec<(isize, isize)>, Vec<Vec<isize>>) {
    let mut split = input.split("-");

    let rules = split.next().unwrap();
    let pages = split.next().unwrap();
    let mut rules_vec = Vec::new();
    let mut pages_vec = Vec::new();

    for line in rules.lines() {
        let mut split = line.split("|");
        let first = split.next().unwrap().parse::<isize>().unwrap();
        let second = split.next().unwrap().parse::<isize>().unwrap();
        rules_vec.push((first, second));
    }

    for line in pages.trim().lines() {
        let mut page = Vec::new();
        for num in line.split(",") {
            page.push(num.parse::<isize>().unwrap());
        }
        pages_vec.push(page);
    }

    (rules_vec, pages_vec)
}

fn compute_correct_middles(global_constraints: &HashMap<isize, PageOrderingConstraints>, pages: Vec<Vec<isize>>) -> (isize, Vec<Vec<isize>>) {
    let mut res = 0;

    let mut wrong_seq = Vec::new();

    for page in pages {
        let mut is_valid = true;
        for (index, val) in page.iter().enumerate() {
            if let Some(constraint) = global_constraints.get(val) {
                let prev = &page[0..index];
                let next = &page[index + 1..];
                if is_valid && prev.iter().all(|x| constraint.before.contains(x)) && next.iter().all(|x| constraint.after.contains(x)) {
                } else {
                    is_valid = false;
                }
            } else {
                // constraint not found, we don't really care then
                continue;
            }
        }
        if is_valid {
            // here, everything is chill -- the sequence is valid
            res += page[page.len()/2];
        } else {
            wrong_seq.push(page);
        }
    }

    (res, wrong_seq)
}

fn fix_and_return_mid(global_constraints: &HashMap<isize, PageOrderingConstraints>, page: Vec<isize>) -> isize {
    let mut fixed = Vec::new();

    let mut is_valid = true;
    for (index, val) in page.iter().enumerate() {
        if let Some(constraint) = global_constraints.get(val) {
            let prev = &page[0..index];
            let next = &page[index + 1..];
            if is_valid && prev.iter().all(|x| constraint.before.contains(x)) && next.iter().all(|x| constraint.after.contains(x)) {
            } else {
                is_valid = false;
            }
        } else {
            // constraint not found, we don't really care then
            continue;
        }
    }

    fixed[fixed.len()/2]
}

fn main() {
    let raw_input = std::fs::read_to_string("five.input.txt").unwrap();

    let (rules, pages) = split_input(&raw_input);

    let global_constraint = parse_rules(&rules);

    println!("sum of all correct middle {}", compute_correct_middles(&global_constraint, pages).0);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13
-
75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
    ";


    #[test]
    fn test_parse_rules() {
        let (rules, _) = split_input(TEST_INPUT);
        let parsed_rules = parse_rules(&rules);

        assert_eq!(rules.len(), 21);
        assert_eq!(parsed_rules.get(&47).unwrap().before, vec![97, 75]);
        assert_eq!(parsed_rules.get(&47).unwrap().after, vec![53, 13, 61, 29]);

    }

    #[test]
    fn test_split_input() {
        let (rules, pages) = split_input(TEST_INPUT);

        assert_eq!(rules.len(), 21);
        assert_eq!(pages.len(), 6);
    }

    #[test]
    fn e2e() {
        let (rules, pages) = split_input(TEST_INPUT);

        let global_constraint = parse_rules(&rules);

        let res = compute_correct_middles(&global_constraint, pages);

        assert_eq!(res.0, 143)
    }

    #[test]
    fn corrected_e2e() {
        let (rules, pages) = split_input(TEST_INPUT);

        let global_constraint = parse_rules(&rules);

        let (_, wrong_seq) = compute_correct_middles(&global_constraint, pages);

        let res: isize = wrong_seq.iter().map(|seq| fix_and_return_mid(&global_constraint, seq.clone())).sum();

        assert_eq!(res, 123)
    }
}