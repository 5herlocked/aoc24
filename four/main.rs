use std::fmt::format;

fn parse_input(file_path: &str) -> Vec<Vec<char>> {
    let input = std::fs::read_to_string(file_path).unwrap();

    input.lines().map(| l | l.chars().collect::<Vec<char>>()).collect()
}

fn xmas_finder(input: &Vec<Vec<char>>, location: (usize, usize)) -> bool {
    // pivot on X
    let lookup_table: Vec<[(isize, isize); 4]> = vec![
        [(0, 0), (0, 1), (0, 2), (0, 3)],  // horizontal
        [(-3, 0), (-2, 0), (-1, 0), (0, 0)],  // vertical X at the bottom
        [(0, 0), (1, 0), (2, 0), (3, 0)],  // vertical X at the top
        [(0, 0), (1, 1), (2, 2), (3, 3)],  // diagonal bottom left to top right
        [(0, 0), (-1, 1), (-2, 2), (-3, 3)],  // diagonal bottom right to top left
        [(0, 0), (1, -1), (2, -2), (3, -3)],  // diagonal top left to bottom right
        [(0, 0), (-1, -1), (-2, -2), (-3, -3)]  // diagonal top right to bottom left
    ];

    for table in lookup_table {
        let mut curr_str = String::new();
        for (x, y) in table {
            let (x, y) = (x + location.0 as isize, y + location.1 as isize);
            if x < 0 || y < 0 || x >= input.len() as isize || y >= input[0].len() as isize {
                continue;
            } else {
                curr_str.push(input[x as usize][y as usize]);
            }
        }

        if curr_str.len() < 4 {
            continue;
        }

        // println!("{}", curr_str);

        if curr_str == "XMAS" || curr_str == "SAMX" {
            return true;
        }
    }

    return false;
}

fn main() {
    // let input_grid = parse_input("four.input.txt");

    let input = "....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX";

    let input_grid=  input.lines().map(| l | l.chars().collect::<Vec<char>>()).collect::<Vec<_>>();

    let num_xmas = input_grid.iter().enumerate().map(|(x, line)| {
        let cloned_grid = input_grid.clone();
        line.iter().enumerate().map(move |(y, c)| {
            if *c != 'X' {
                return false;
            }
            xmas_finder(&cloned_grid, (x, y))
        })
    }).flatten().filter(|is_xmas| *is_xmas).collect::<Vec<_>>().len();

    println!("XMAS Found: {} times", num_xmas);
}