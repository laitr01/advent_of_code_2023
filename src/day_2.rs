#[path = "utils/file.rs"]
mod file;
#[path = "utils/number.rs"]
mod number;

use std::str::FromStr;
use std::collections::HashMap;

/*
--- Day 2: I Was Told There Would Be No Math ---
The elves are running low on wrapping paper, and so they need to submit an order for more. They have a list of the dimensions (range l, width w, and height h) of each present, and only want to order exactly as much as they need.

Fortunately, every present is a box (a perfect right rectangular prism), which makes calculating the required wrapping paper for each gift a little easier: find the surface area of the box, which is 2*l*w + 2*w*h + 2*h*l. The elves also need a little extra paper for each present: the area of the smallest side.

For example:

A present with dimensions 2x3x4 requires 2*6 + 2*12 + 2*8 = 52 square feet of wrapping paper plus 6 square feet of slack, for a total of 58 square feet.
A present with dimensions 1x1x10 requires 2*1 + 2*10 + 2*10 = 42 square feet of wrapping paper plus 1 square foot of slack, for a total of 43 square feet.
All numbers in the elves' list are in feet. How many total square feet of wrapping paper should they order?
*/
pub fn solve_the_puzzle_2_1() {
    match file::read_file("inputs/day_2_1.txt") {
        Err(error) => {
            println!("error {}", error)
        }
        Ok(contents) => {
            //println!("{}", contents)
            let lines: Vec<&str> = contents.lines().collect();
            let mut total: i32 = 0;
            for line in lines {
                let num_str: Vec<&str> = line.split('x').collect();
                let l = i32::from_str(num_str[0]).unwrap();
                let w = i32::from_str(num_str[1]).unwrap();
                let h = i32::from_str(num_str[2]).unwrap();
                total += 2 * l * w + 2 * w * h + 2 * h * l;
                let min = number::min_of_three(l * w, w * h, h * l);
                total += min;

//                println!("{} {} {}", l, w,h);
//                println!("{}", min);
//                println!("{}", total);
            }
            println!("{}", total);
        }
    }
}

/*
--- Day 2: Cube Conundrum ---
You're launched high into the atmosphere! The apex of your trajectory just barely reaches the surface of a large island floating in the sky. You gently land in a fluffy pile of leaves.
It's quite cold, but you don't see much snow. An Elf runs over to greet you.
The Elf explains that you've arrived at Snow Island and apologizes for the lack of snow. He'll be happy to explain the situation, but it's a bit of a walk, so you have some time.
They don't get many visitors up here; would you like to play a game in the meantime?
As you walk, the Elf shows you a small bag and some cubes which are either red, green, or blue.
Each time you play this game, he will hide a secret number of cubes of each color in the bag, and your goal is to figure out information about the number of cubes.
To get information, once a bag has been loaded with cubes, the Elf will reach into the bag, grab a handful of random cubes, show them to you, and then put them back in the bag. He'll do this a few times per game.
You play several games and record the information from each game (your puzzle input). Each game is listed with its ID number (like the 11 in Game 11: ...)
followed by a semicolon-separated list of subsets of cubes that were revealed from the bag (like 3 red, 5 green, 4 blue).
For example, the record of a few games might look like this:
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
In game 1, three sets of cubes are revealed from the bag (and then put back again).
The first set is 3 blue cubes and 4 red cubes; the second set is 1 red cube, 2 green cubes, and 6 blue cubes; the third set is only 2 green cubes.
The Elf would first like to know which games would have been possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?
In the example above, games 1, 2, and 5 would have been possible if the bag had been loaded with that configuration.
However, game 3 would have been impossible because at one point the Elf showed you 20 red cubes at once; similarly,
game 4 would also have been impossible because the Elf showed you 15 blue cubes at once.
If you add up the IDs of the games that would have been possible, you get 8.
Determine which games would have been possible if the bag had been loaded with only 12 red cubes, 13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games?
*/
pub fn solve_the_puzzle_2_2() {
    const GREEN: i32 = 13;
    const RED: i32 = 12;
    const BLUE: i32 = 14;

    match file::read_file("inputs/day_2_2.txt") {
        Err(error) => {
            println!("error {}", error)
        }
        Ok(contents) => {
            let games: Vec<&str> = contents.lines().collect();
            let mut result: i32 = 0;
            for (i, game) in games.iter().enumerate() {
                let trim_title: Vec<&str> = game.split(':').collect();
                let subsets: Vec<&str> = trim_title[1].split(';').collect();
                //println!("{}", trim_title[1]);
                let mut possible: bool = true;
                for (_set, subset) in subsets.iter().enumerate() {
                    let cube_set: Vec<&str> = subset.split(',').collect();
                    for (_time, cube) in cube_set.iter().enumerate() {
                        let cube_set_1: Vec<&str> = cube.trim().split(' ').collect();
                        let key: &str = cube_set_1[1];
                        let value: i32 = cube_set_1[0].parse::<i32>().unwrap();
                        //println!("set {} - time {}: {} {}", set, time, key, value);
                        match key {
                            "green" => {
                                if value > GREEN {
                                    possible = false;
                                }
                            }
                            "red" => {
                                if value > RED {
                                    possible = false;
                                }
                            }
                            "blue" => {
                                if value > BLUE {
                                    possible = false;
                                }
                            }
                            _ => {
                                // Ignore keys that are not "green", "red", or "blue"
                            }
                        }
                    }
                }
                if possible {
                   // println!("possible {}", i + 1);
                    result += (i + 1) as i32;
                } else {
                   // println!("impossible {}", i + 1);
                }
            }
            println!("{}", result);
        }
    }
}
/*
--- Part Two ---
The Elf says they've stopped producing snow because they aren't getting any water!
He isn't sure why the water stopped; however, he can show you how to get to the water source to check it out for yourself. It's just up ahead!
As you continue your walk, the Elf poses a second question: in each game you played,
what is the fewest number of cubes of each color that could have been in the bag to make the game possible?
Again consider the example games from earlier:
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
In game 1, the game could have been played with as few as 4 red, 2 green, and 6 blue cubes.
If any color had even one fewer cube, the game would have been impossible.
Game 2 could have been played with a minimum of 1 red, 3 green, and 4 blue cubes.
Game 3 must have been played with at least 20 red, 13 green, and 6 blue cubes.
Game 4 required at least 14 red, 3 green, and 15 blue cubes.
Game 5 needed no fewer than 6 red, 3 green, and 2 blue cubes in the bag.
The power of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied together.
The power of the minimum set of cubes in game 1 is 48. In games 2-5 it was 12, 1560, 630, and 36, respectively.
Adding up these five powers produces the sum 2286.
For each game, find the minimum set of cubes that must have been present. What is the sum of the power of these sets?
*/
pub fn solve_the_puzzle_2_3() {
    match file::read_file("inputs/day_2_2.txt") {
        Err(error) => {
            println!("error {}", error)
        }
        Ok(contents) => {
            let games: Vec<&str> = contents.lines().collect();
            let mut result: i32 = 0;
            for (_i, game) in games.iter().enumerate() {
                let trim_title: Vec<&str> = game.split(':').collect();
                let subsets: Vec<&str> = trim_title[1].split(';').collect();
                let mut required_conditions_map = HashMap::new();
                //println!("{}", trim_title[1]);
                for (_set, subset) in subsets.iter().enumerate() {
                    let cube_set: Vec<&str> = subset.split(',').collect();
                    for (_time, cube) in cube_set.iter().enumerate() {
                        let cube_set_1: Vec<&str> = cube.trim().split(' ').collect();
                        let key: &str = cube_set_1[1];
                        let value: i32 = cube_set_1[0].parse::<i32>().unwrap();
                        //println!("set {} - time {}: {} {}", set, time, key, value);
                        match required_conditions_map.get(key) {
                            Some(v) => {
                                if value > *v {
                                    required_conditions_map.insert(key, value);
                                }
                            }
                            None => {
                                required_conditions_map.insert(key, value);
                            }
                        }
                    }
                }
                let mut result_set = 1;
                for (_k,v) in required_conditions_map.iter() {
                    result_set *= v;
                    //println!("{}: {} {} {}", i, k, v, result_set);
                }
                result+=result_set;
            }
            println!("{}", result);
        }
    }
}