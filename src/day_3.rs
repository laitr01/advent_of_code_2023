#[path = "utils/file.rs"]
mod file;

use std::collections::HashMap;
use std::fmt;

/*
--- Day 3: Gear Ratios ---
You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.
It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving
"Aaah!"
You turn around to see a slightly-greasy Elf with a wrench and a look of surprise.
"Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.
The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one.
If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.
The engine schematic (your puzzle input) consists of a visual representation of the engine.
There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (
Periods (.) do not count as a symbol.)
Here is an example engine schematic:
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right).
Every other number is adjacent to a symbol and so is a part number; their sum is 4361.
Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?
*/
#[derive(Debug)]
struct NumLoc {
    row: usize,
    col: usize,
}

impl fmt::Display for NumLoc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.row, self.col)
    }
}

pub fn solve_the_puzzle_3_1() {
    match file::read_file("inputs/day_3_1.txt") {
        Err(error) => {
            println!("error {}", error)
        }
        Ok(schematic) => {
            let mut row_count = 0;
            let mut col_count = 0;
            for line in schematic.lines() {
                if row_count == 0 {
                    let cols: Vec<char> = line.chars().clone().into_iter().collect();
                    col_count = cols.len();
                }
                row_count += 1;
            }
            let mut sum = 0;
            let mut symbols = vec![vec!['.'; col_count]; row_count];
            let mut number_parts: HashMap<usize, i32> = HashMap::new();
            let mut symbol_adjacent: Vec<NumLoc> = Vec::new();

            for (row, line) in schematic.lines().enumerate() {
                for (col, ch) in line.chars().enumerate() {
                    if !ch.is_digit(10) && ch != '.' {
                        for i in row.saturating_sub(1)..=row + 1 {
                            for j in col.saturating_sub(1)..=col + 1 {
                                if i < row || i > row || j < col || j > col {
                                    symbol_adjacent.push(NumLoc { row: i, col: j });
                                }
                            }
                        }
                        //println!("{} ({},{}) -> {:?}", ch, row, col, symbol_adjacent);
                    }
                    symbols[row][col] = ch;
                }
            }
            for adj in symbol_adjacent {
                let c = symbols[adj.row][adj.col];
                if c.is_digit(10) {
                    let mut number = format!("{}", c);
                    let mut start = adj.col as usize;
                    let mut left = start - 1;
                    let mut right = start + 1;
                    while left >= 0 {
                        //println!("{}", left);
                        let l = symbols[adj.row][left];
                        if l.is_digit(10) {
                            number = format!("{}{}", l, number);
                            start = left;
                            if left == 0 {
                                break;
                            }
                            left -= 1;
                        } else { break; }
                    }
                    while right < col_count {
                        let r = symbols[adj.row][right];
                        if r.is_digit(10) {
                            number = format!("{}{}", number, r);
                            right += 1;
                        } else { break; }
                    }
                    let num = number.parse::<i32>().unwrap();
                    let key = start + col_count * adj.row;//row * num_columns + column
                    if !number_parts.contains_key(&key) {
                        number_parts.insert(key, num);
                    }
                }
            }
            for (_k, v) in number_parts.iter() {
                sum += v;
            }
            //println!("{:?}", number_parts);
            println!("The sum of all part numbers in the engine schematic is {}", sum);
        }
    }
}

/*
--- Part Two ---
The engineer finds the missing part and installs it in the engine! As the engine springs to life, you jump in the closest gondola, finally ready to ascend to the water source.
You don't seem to be going very fast, though. Maybe something is still wrong? Fortunately, the gondola has a phone labeled "help", so you pick it up and the engineer answers.
Before you can explain the situation, she suggests that you look out the window. There stands the engineer, holding a phone in one hand and waving with the other.
You're going so slowly that you haven't even left the station. You exit the gondola.
The missing part wasn't the only issue - one of the gears in the engine is wrong. A gear is any * symbol that is adjacent to exactly two part numbers.
 Its gear ratio is the result of multiplying those two numbers together.
This time, you need to find the gear ratio of every gear and add them all up so that the engineer can figure out which gear needs to be replaced.
Consider the same engine schematic again:
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
In this schematic, there are two gears. The first is in the top left; it has part numbers 467 and 35, so its gear ratio is 16345. The second gear is in the lower right; its gear ratio is 451490.
(The * adjacent to 617 is not a gear because it is only adjacent to one part number.) Adding up all of the gear ratios produces 467835.
What is the sum of all of the gear ratios in your engine schematic?
*/
pub fn solve_the_puzzle_3_2() {
    match file::read_file("inputs/day_3_1.txt") {
        Err(error) => {
            println!("error {}", error)
        }
        Ok(schematic) => {
            let mut row_count = 0;
            let mut col_count = 0;
            for line in schematic.lines() {
                if row_count == 0 {
                    let cols: Vec<char> = line.chars().clone().into_iter().collect();
                    col_count = cols.len();
                }
                row_count += 1;
            }
            let mut sum = 0;
            let mut symbols = vec![vec!['.'; col_count]; row_count];
            let mut gear_map: HashMap<usize, Vec<NumLoc>> = HashMap::new();

            for (row, line) in schematic.lines().enumerate() {
                for (col, ch) in line.chars().enumerate() {
                    if ch == '*' {
                        let mut gear_loc: Vec<NumLoc> = Vec::new();
                        for i in row.saturating_sub(1)..=row + 1 {
                            for j in col.saturating_sub(1)..=col + 1 {
                                if i < row || i > row || j < col || j > col {
                                    gear_loc.push(NumLoc { row: i, col: j });
                                }
                            }
                        }
                        let key = col + col_count * row;
                        gear_map.insert(key, gear_loc);
                    }
                    symbols[row][col] = ch;
                }
            }
            //println!("{:?}", gear_map);
            let mut gear_loc_map: HashMap<usize, HashMap<usize, i32>> = HashMap::new();
            for (k, gears_loc) in gear_map.iter() {
                let mut loc_map: HashMap<usize, i32> = HashMap::new();
                for num_loc in gears_loc {
                    let c = symbols[num_loc.row][num_loc.col];
                    if c.is_digit(10) {
                        let mut number = format!("{}", c);
                        let mut start = num_loc.col as usize;
                        let mut left = start - 1;
                        let mut right = start + 1;
                        while left >= 0 {
                            //println!("{}", left);
                            let l = symbols[num_loc.row][left];
                            if l.is_digit(10) {
                                number = format!("{}{}", l, number);
                                start = left;
                                if left == 0 {
                                    break;
                                }
                                left -= 1;
                            } else { break; }
                        }
                        while right < col_count {
                            let r = symbols[num_loc.row][right];
                            if r.is_digit(10) {
                                number = format!("{}{}", number, r);
                                right += 1;
                            } else { break; }
                        }
                        let num = number.parse::<i32>().unwrap();
                        let key = start + col_count * num_loc.row;//row * num_columns + column
                        loc_map.insert(key, num);
                    }
                }
                gear_loc_map.insert(*k, loc_map);
            }
            //println!("{:?}",gear_loc_map);
            for (_k, v) in gear_loc_map.iter() {
                if v.len() == 2 {
                    let mut mul = 1;
                    for (_k, n) in v.iter() {
                        mul *= n;
                    }
                    sum += mul;
                    //println!("{:?},{}", v, sum);
                }
            }
            //println!("{:?}", gear_map);
            println!("The sum of all part numbers in the engine schematic is {}", sum);
        }
    }
}