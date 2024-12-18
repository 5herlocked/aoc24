use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Guard {
    x: i32,
    y: i32,
    direction: (i32, i32)
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Location {
    x: i32,
    y: i32,
}

fn parse_map(input: &str) -> (Vec<Location>, Guard, i32, i32) {
    let mut blocks = Vec::new();
    let mut guard = Guard { x: 0, y: 0, direction: (0, -1) };

    let lim_x = input.lines().count() as i32;
    let lim_y = input.lines().next().unwrap().chars().count() as i32;

    for (x, line) in input.lines().enumerate() {
        for (y, char) in line.chars().enumerate() {
            match char {
                '#' => blocks.push(Location { x: x as i32, y: y as i32 }),
                '>' => {
                    guard.direction = (0, 1);
                    guard.x = x as i32;
                    guard.y = y as i32;
                },
                '<' => {
                    guard.direction = (0, -1);
                    guard.x = x as i32;
                    guard.y = y as i32;
                },
                '^' => {
                    guard.direction = (-1, 0);
                    guard.x = x as i32;
                    guard.y = y as i32;
                },
                'v' => { 
                    guard.direction = (1, 0);
                    guard.x = x as i32;
                    guard.y = y as i32;
                },
                _ => (),
            }
        }
    }

    (blocks, guard, lim_x, lim_y)
}

// runs the full simulation of the guard moving about in the map
// returns how many unique blocks were covered
fn simulate(map: Vec<Location>, mut char: Guard, lim_x: i32, lim_y: i32) -> usize {
    let mut visited: HashSet<Location> = HashSet::new();

    visited.insert(Location { x: char.x, y: char.y });  // starting point

    while char.x < lim_x || char.y < lim_y {
        // check if the block in front of the guard is blocked
        let next_pos = Location {
            x: char.x + char.direction.0,
            y: char.y + char.direction.1,
        };

        if map.contains(&next_pos) {
            // turn right
            char.direction = match char.direction {
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                (-1, 0) => (0, 1),
                _ => panic!("Invalid direction"),
            }
        }

        // move the guard
        char.x += char.direction.0;
        char.y += char.direction.1;

        println!("{}, {}", char.x, char.y);

        visited.insert(Location { x: char.x, y: char.y });

        // check if the guard is out of bounds
        if char.x < 0 || char.y < 0 || char.x >= (lim_x - 1) || char.y >= (lim_y - 1) {
            break;
        }
    }

    visited.len() as usize
}

fn main() -> () {

    let raw_input = std::fs::read_to_string("six.input.txt").expect("Error reading input.txt");

    let (map, guard, lim_x, lim_y) = parse_map(&raw_input);

    let result = simulate(map.clone(), guard.clone(), lim_x, lim_y);

    println!("Result: {}", result);
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_parse_map() {
        let (map, guard, lim_x, lim_y) = parse_map(TEST_INPUT);

        assert!(map.len() == 8);
        assert!(guard.x == 6);
        assert!(guard.y == 4);
        assert!(lim_x == 10);
        assert!(lim_y == 10);
    }

    #[test]
    fn test_simulate() {
        let (map, guard, lim_x, lim_y) = parse_map(TEST_INPUT);

        let res = simulate(map, guard, lim_x, lim_y);

        println!("Test result: {}", res);

        assert!(res == 41)
    }
}