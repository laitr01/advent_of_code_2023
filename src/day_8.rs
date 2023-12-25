#[path = "utils/file.rs"]
mod file;

use std::collections::HashMap;

/**
--- Day 8: Haunted Wasteland ---
You're still riding a camel across Desert Island when you spot a sandstorm quickly approaching.
When you turn to warn the Elf, she disappears before your eyes! To be fair, she had just finished warning you about ghosts a few minutes ago.
One of the camel's pouches is labeled "maps" - sure enough, it's full of documents (your puzzle input) about how to navigate the desert.
At least, you're pretty sure that's what they are; one of the documents contains a list of left/right instructions, and the rest of the documents seem to describe some kind of network of labeled nodes.
It seems like you're meant to use the left/right instructions to navigate the network. Perhaps if you have the camel follow the same instructions, you can escape the haunted wasteland!
After examining the maps for a bit, two nodes stick out: AAA and ZZZ. You feel like AAA is where you are now, and you have to follow the left/right instructions until you reach ZZZ.
This format defines each node of the network individually. For example:
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
Starting with AAA, you need to look up the next element based on the next left/right instruction in your input. In this example, start with AAA and go right (R) by choosing the right element of AAA, CCC.
Then, L means to choose the left element of CCC, ZZZ. By following the left/right instructions, you reach ZZZ in 2 steps.
Of course, you might not find ZZZ right away. If you run out of left/right instructions, repeat the whole sequence of instructions as necessary: RL really means RLRLRLRLRLRLRLRL... and so on.
For example, here is a situation that takes 6 steps to reach ZZZ:
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
Starting at AAA, follow the left/right instructions. How many steps are required to reach ZZZ?
*/
#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

impl Node {
    pub fn pick(&self, c: &char) -> &str {
        if c == &'L' {
            return &self.left;
        } else if c == &'R' {
            return &self.right;
        } else {
            panic!("Invalid char");
        }
    }
}

fn steps(path: Vec<char>, graph: HashMap<String, Node>, start: String, goal: String) -> usize {
    let mut node = start;
    1 + path.iter().cycle().position(|&d| {
        node = graph[&node].pick(&d).to_string();
        node.ends_with(&goal)
    }).unwrap()
}

fn parse(line: &str) -> (String, Node) {
    let parts: Vec<&str> = line.split(" = ").collect();
    let key = parts[0].to_string();
    let value = parts[1].to_string();
    let values: Vec<&str> = value
        .trim_matches(|c| c == '(')
        .trim_matches(|c| c == ')')
        .split(',')
        .map(|v| v.trim())
        .collect();
    let right = values[1].strip_suffix(')').unwrap_or(values[1]);
    let node = Node { left: values[0].to_string(), right: right.to_string() };

    (key, node)
}

pub fn solve_the_puzzle_8_1() {
    match file::read_file("inputs/day_8_1.txt") {
        Err(error) => {
            println!("error {}", error)
        }
        Ok(input) => {
            let parts: Vec<&str> = input.split("\r\n\r\n").collect();
            let instructions: Vec<char> = parts[0].chars().collect();
            let mut graph = HashMap::new();

            let pairs: Vec<&str> = parts[1].split("\n").filter(|s| !s.is_empty()).collect();
            for pair in pairs {
                let (key, node) = parse(pair);
                graph.insert(key.to_string(), node);
            }
            let result = steps(instructions, graph, "AAA".to_string(), "ZZZ".to_string());
            println!("{}", result);
        }
    }
}
/**
--- Part Two ---
The sandstorm is upon you and you aren't any closer to escaping the wasteland. You had the camel follow the instructions, but you've barely left your starting position.
It's going to take significantly more steps to escape!
What if the map isn't for people - what if the map is for ghosts? Are ghosts even bound by the laws of spacetime? Only one way to find out.
After examining the maps a bit longer, your attention is drawn to a curious fact: the number of nodes with names ending in A is equal to the number ending in Z!
If you were a ghost, you'd probably just start at every node that ends with A and follow all of the paths at the same time until they all simultaneously end up at nodes that end with Z.
For example:
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
Here, there are two starting nodes, 11A and 22A (because they both end with A).
As you follow each left/right instruction, use that instruction to simultaneously navigate away from both nodes you're currently on.
Repeat this process until all of the nodes you're currently on end with Z. (If only some of the nodes you're on end with Z, they act like any other node and you continue as normal.)
In this example, you would proceed as follows:
Step 0: You are at 11A and 22A.
Step 1: You choose all of the left paths, leading you to 11B and 22B.
Step 2: You choose all of the right paths, leading you to 11Z and 22C.
Step 3: You choose all of the left paths, leading you to 11B and 22Z.
Step 4: You choose all of the right paths, leading you to 11Z and 22B.
Step 5: You choose all of the left paths, leading you to 11B and 22C.
Step 6: You choose all of the right paths, leading you to 11Z and 22Z.
So, in this example, you end up entirely on nodes that end in Z after 6 steps.
Simultaneously start on every node that ends with A. How many steps does it take before you're only on nodes that end with Z?
*/
pub fn solve_the_puzzle_8_2() {
    match file::read_file("inputs/day_8_1.txt") {
        Err(error) => {
            println!("error {}", error)
        }
        Ok(input) => {
            let parts: Vec<&str> = input.split("\r\n\r\n").collect();
            let instructions: Vec<char> = parts[0].chars().collect();
            let mut graph = HashMap::new();

            let pairs: Vec<&str> = parts[1].split("\n").filter(|s| !s.is_empty()).collect();
            for pair in pairs {
                let (key, node) = parse(pair);
                graph.insert(key.to_string(), node);
            }
            let result = graph.keys()
                .filter(|k| k.ends_with("A"))
                .map(|node| steps(instructions, graph, node.to_string(), "Z".to_string()))
                .fold(1, |ans, x| (x*ans)/gcd(x, ans));
            println!("{}", result);
        }
    }
}

fn gcd(a: usize, b: usize) -> usize {
    // Base cases
    if b == 0 {
        return a;
    }

    if a == 0 {
        return b;
    }

    // Find the greatest power of 2 that divides both a and b
    let shift = (a | b).trailing_zeros();

    // Divide both a and b by 2 until their greatest power of 2 is different
    let mut a = a >> a.trailing_zeros();
    let mut b = b >> b.trailing_zeros();

    while a != b {
        if a > b {
            a -= b;
            a >>= a.trailing_zeros();
        } else {
            b -= a;
            b >>= b.trailing_zeros();
        }
    }

    // Multiply the result by 2^shift to get the final GCD
    a << shift
}


