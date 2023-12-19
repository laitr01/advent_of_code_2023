#[path = "utils/file.rs"]
mod file;

/*
--- Day 6: Wait For It ---
The ferry quickly brings you across Island Island. After asking around, you discover that there is indeed normally a large pile of sand somewhere near here, but you don't see anything besides lots of water and the small island where the ferry has docked.
As you try to figure out what to do next, you notice a poster on a wall near the ferry dock.
"Boat races! Open to the public! Grand prize is an all-expenses-paid trip to Desert Island!" That must be where the sand comes from! Best of all, the boat races are starting in just a few minutes.
You manage to sign up as a competitor in the boat races just in time. The organizer explains that it's not really a traditional race - instead, you will get a fixed amount of time during which your boat has to travel as far as it can, and you win if your boat goes the farthest.
As part of signing up, you get a sheet of paper (your puzzle input) that lists the time allowed for each race and also the best distance ever recorded in that race.
To guarantee you win the grand prize, you need to make sure you go farther in each race than the current record holder.
The organizer brings you over to the area where the boat races are held. The boats are much smaller than you expected - they're actually toy boats, each with a big button on top.
Holding down the button charges the boat, and releasing the button allows the boat to move.
Boats move faster if their button was held longer, but time spent holding the button counts against the total race time.
You can only hold the button at the start of the race, and boats don't move until the button is released.
For example:
Time:      7  15   30
Distance:  9  40  200
This document describes three races:
The first race lasts 7 milliseconds. The record distance in this race is 9 millimeters.
The second race lasts 15 milliseconds. The record distance in this race is 40 millimeters.
The third race lasts 30 milliseconds. The record distance in this race is 200 millimeters.
Your toy boat has a starting speed of zero millimeters per millisecond.
For each whole millisecond you spend at the beginning of the race holding down the button, the boat's speed increases by one millimeter per millisecond.
So, because the first race lasts 7 milliseconds, you only have a few options:
Don't hold the button at all (that is, hold it for 0 milliseconds) at the start of the race. The boat won't move; it will have traveled 0 millimeters by the end of the race.
Hold the button for 1 millisecond at the start of the race. Then, the boat will travel at a speed of 1 millimeter per millisecond for 6 milliseconds, reaching a total distance traveled of 6 millimeters.
Hold the button for 2 milliseconds, giving the boat a speed of 2 millimeters per millisecond. It will then get 5 milliseconds to move, reaching a total distance of 10 millimeters.
Hold the button for 3 milliseconds. After its remaining 4 milliseconds of travel time, the boat will have gone 12 millimeters.
Hold the button for 4 milliseconds. After its remaining 3 milliseconds of travel time, the boat will have gone 12 millimeters.
Hold the button for 5 milliseconds, causing the boat to travel a total of 10 millimeters.
Hold the button for 6 milliseconds, causing the boat to travel a total of 6 millimeters.
Hold the button for 7 milliseconds. That's the entire duration of the race. You never let go of the button. The boat can't move until you let go of the button. Please make sure you let go of the button so the boat gets to move. 0 millimeters.
Since the current record for this race is 9 millimeters, there are actually 4 different ways you could win: you could hold the button for 2, 3, 4, or 5 milliseconds at the start of the race.
In the second race, you could hold the button for at least 4 milliseconds and at most 11 milliseconds and beat the record, a total of 8 different ways to win.
In the third race, you could hold the button for at least 11 milliseconds and no more than 19 milliseconds and still beat the record, a total of 9 ways you could win.
To see how much margin of error you have, determine the number of ways you can beat the record in each race; in this example, if you multiply these values together, you get 288 (4 * 8 * 9).
Determine the number of ways you could beat the record in each race. What do you get if you multiply these numbers together?
*/
#[derive(Debug)]
struct RaceRecord {
    time: usize,
    distance: usize,
}
impl RaceRecord {
    fn from(time: usize, distance: usize) -> RaceRecord {
        return RaceRecord {
            time,
            distance,
        };
    }
    pub fn from_str(lines: &str) -> RaceRecord {
        let time_str = lines
            .lines()
            .nth(0)
            .unwrap()
            .split(':')
            .nth(1)
            .unwrap()
            .replace(" ", "");

        let distance_str = lines
            .lines()
            .nth(1)
            .unwrap()
            .split(':')
            .nth(1)
            .unwrap()
            .replace(" ", "");
//        println!("{}", time_str);
//        println!("{}", distance_str);
        let time: usize = time_str.parse().unwrap();
        let distance: usize = distance_str.parse().unwrap();

        RaceRecord { time, distance }
    }
}
#[derive(Debug)]
struct RaceRecords {
    list: Vec<RaceRecord>,
}

impl RaceRecords {
    pub fn from_str(lines: &str) -> RaceRecords {
        let times: Vec<usize> = lines
            .lines()
            .nth(0)
            .unwrap()
            .split(':')
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let distances: Vec<usize> = lines
            .lines()
            .nth(1)
            .unwrap()
            .split(':')
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let records: Vec<RaceRecord> = times
            .iter()
            .zip(distances.iter())
            .map(|(&time, &distance)| RaceRecord::from(time, distance))
            .collect();

        RaceRecords { list: records }
    }
}

pub fn solve_the_puzzle_6_1() {
    match file::read_file("inputs/day_6_1.txt") {
        Err(error) => {
            println!("error {}", error)
        }
        Ok(input) => {
            let race_records: RaceRecords = RaceRecords::from_str(&input);
            let mut number_of_way = 1;
            for record in race_records.list {
                let mut count = 0;
                for i in 0..record.time {
                    let r = i * (record.time-i);
                    if r > record.distance {
                        count+=1;
                    }
                }
                number_of_way *= count;
            }
            println!("Number of way 1: {}", number_of_way);
        }
    }
}

pub fn solve_the_puzzle_6_2() {
    match file::read_file("inputs/day_6_1.txt") {
        Err(error) => {
            println!("error {}", error)
        }
        Ok(input) => {
            let record: RaceRecord = RaceRecord::from_str(&input);
            let mut total_ways = 0;
            for i in 0..record.time {
                let r = i * (record.time-i);
                if r > record.distance {
                    total_ways+=1;
                }
            }
            println!("Number of way 2: {}", total_ways);
        }
    }
}


