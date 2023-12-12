use std::collections::HashMap;

use gcd::Gcd;
use regex::Regex;

#[cfg(test)]
const INPUT: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

#[test]
fn test1() {
    assert_eq!(part1(parse(INPUT)), 6);
}

#[cfg(test)]
const INPUT2: &str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

#[test]
fn test2() {
    assert_eq!(part2(parse(INPUT2)), 6);
}


fn main() {
    let input = std::fs::read_to_string("./input/day8.txt").unwrap();
    
    println!("Part 1 solution: {}", part1(parse(&input)));
    println!("Part 2 solution: {}", part2(parse(&input)));
}


struct Input {
    directions: Vec<Direction>,
    nodes: HashMap<String, (String, String)>
}

enum Direction { Left, Right }

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(())
        }
    }
}

fn parse(input: &str) -> Input {
    let mut lines = input.lines();
    let directions = lines.next().unwrap().chars().map(Direction::try_from).map(Result::unwrap).collect();
    lines.next();

    let mut result = Input {
        directions,
        nodes: HashMap::new()
    };

    let regex = Regex::new("(\\w{3}) = \\((\\w{3}), (\\w{3})\\)").unwrap();
    for line in lines {
        let captures = regex.captures(line).unwrap();
        result.nodes.insert(captures[1].to_owned(), (captures[2].to_owned(), captures[3].to_owned()));
    }

    result
}

fn part1(input: Input) -> u32 {
    let mut node = "AAA";
    let mut steps = 0;

    for choice in input.directions.iter().cycle() {
        node = match choice {
            Direction::Left => &input.nodes[node].0,
            Direction::Right => &input.nodes[node].1,
        };
        steps += 1;

        if node == "ZZZ" {
            break;
        }
    }

    steps
}

#[derive(Debug)]
struct NodeInfo {
    _offset: usize,
    cycle: usize,
    _goals: Vec<usize>,
}

fn get_node_info<'a>(mut node: &'a str, input: &'a Input) -> NodeInfo {
    let mut seen_nodes: Vec<(&str, usize)> = Vec::new();
    let mut steps = 0usize;

    let (_, initial, cycle) = loop {
        let i_dir = steps % input.directions.len();
        
        if node.ends_with('Z') {
            if let Some((i, (_, s))) = seen_nodes.iter().copied().enumerate().find(|&(_, (n, s))| n == node && s % input.directions.len() == i_dir) {
                break (i, s, steps - s);
            }
            seen_nodes.push((node, steps));
        }
        
        node = match input.directions[i_dir] {
            Direction::Left => &input.nodes[node].0,
            Direction::Right => &input.nodes[node].1,
        };
        
        steps += 1;
    };

    //println!("Seen: {seen_nodes:?}");

    NodeInfo {
        _offset: initial,
        cycle: cycle,
        _goals: seen_nodes.iter().map(|n| n.1).collect(),
    }
}

fn part2(input: Input) -> usize {
    let start_nodes: Vec<_> =  input.nodes.keys().filter(|k| k.ends_with('A')).map(String::as_str).collect();
    let infos: Vec<_> = start_nodes.iter().map(|n| get_node_info(*n, &input)).collect();

    let scm = infos.iter().map(|i| i.cycle).reduce(|a, b| {
        a * b / a.gcd(b)
    }).unwrap();

    scm
}