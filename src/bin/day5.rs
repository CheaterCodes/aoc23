fn main() {
    let input = std::fs::read_to_string("./input/day5.txt").unwrap();
    
    println!("Part 1 solution: {}", part1(parse(&input)));
    println!("Part 2 solution: {}", part2(parse(&input)));
}

#[derive(Debug)]
struct Input {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

#[derive(Debug)]
struct Map {
    _from: String,
    _to: String,
    entries: Vec<Entry>,
}

#[derive(Debug)]
struct Entry {
    destination: usize,
    source: usize,
    length: usize,
}

fn parse(input: &str) -> Input {
    let mut lines = input.lines();

    let seeds = lines.next().unwrap()[6..].split_whitespace().map(str::parse).map(Result::unwrap).collect();
    lines.next();

    let mut input = Input {
        seeds: seeds,
        maps: vec![]
    };

    while let Some(header) = lines.next() {
        let mut header = header.split_once(' ').unwrap().0.split('-');
        let mut map = Map {
            _from: header.next().unwrap().to_owned(),
            _to: header.next_back().unwrap().to_owned(),
            entries: vec![],
        };

        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }

            let mut parts = line.split_whitespace().map(str::parse).map(Result::unwrap);
            map.entries.push(Entry {
                destination: parts.next().unwrap(),
                source: parts.next().unwrap(),
                length: parts.next().unwrap()
            });
        }
        
        input.maps.push(map);
    }

    input
}

fn part1(input: Input) -> u32 {

    let mut indices = input.seeds.clone();

    for map in input.maps {
        for index in &mut indices {
            for entry in &map.entries {
                if *index >= entry.source && *index - entry.source< entry.length {
                    *index = *index - entry.source + entry.destination;
                    break;
                }
            }
        }
    }

    *indices.iter().min().unwrap() as u32
}

fn part2(_input: Input) -> u32 {
    0
}

#[cfg(test)]
const INPUT: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

#[test]
fn test1() {
    assert_eq!(part1(parse(INPUT)), 35);
}

#[test]
fn test2() {
    assert_eq!(part2(parse(INPUT)), 0);
}