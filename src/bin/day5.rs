use std::ops::Range;

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
    source: Range<usize>,
    destination: Range<usize>,
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
            
            let destination: usize = parts.next().unwrap();
            let source: usize = parts.next().unwrap();
            let length: usize = parts.next().unwrap();

            map.entries.push(Entry {
                source: source .. source + length,
                destination: destination .. destination + length,
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
                if entry.source.contains(index) {
                    *index = *index - entry.source.start + entry.destination.start;
                    break;
                }
            }
        }
    }

    *indices.iter().min().unwrap() as u32
}

fn part2(input: Input) -> usize {
    let mut ranges: Vec<_> = input.seeds.chunks_exact(2).map(|c| (c[0] .. c[0] + c[1])).collect();
    
    for map in input.maps {
        let mut mapped_ranges = Vec::new();

        'range: while let Some(range) = ranges.pop() {
            if range.is_empty() {
                continue;
            }

            for entry in &map.entries {
                // Fully contained
                if entry.source.start <= range.start && range.end <= entry.source.end {
                    mapped_ranges.push((range.start - entry.source.start + entry.destination.start) .. (range.end - entry.source.start + entry.destination.start));
                    continue 'range;
                }

                // Contains start
                if entry.source.contains(&range.start) {
                    ranges.push(range.start .. entry.source.end);
                    ranges.push(entry.source.end .. range.end);
                    continue 'range;
                }
                
                // Contains end
                if entry.source.contains(&(range.end - 1)) {
                    ranges.push(range.start .. entry.source.start);
                    ranges.push(entry.source.start .. range.end);
                    continue 'range;
                }
            }

            mapped_ranges.push(range);
        }

        ranges = mapped_ranges;
    }

    ranges.iter().map(|r| r.start).min().unwrap()
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
    assert_eq!(part2(parse(INPUT)), 46);
}