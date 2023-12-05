use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("./input/day3.txt").unwrap();

    
    println!("Part 1 solution: {}", part1(&input));
    println!("Part 2 solution: {}", part2(&input));
}

fn part1(input: &str) -> u32 {
    let number_regex = Regex::new("\\d+").unwrap();
    let symbol_regex = Regex::new("[^\\.\\d]").unwrap();

    let mut numbers = Vec::new();
    let mut symbols = Vec::new();

    for line in input.lines() {
        let line_numbers: Vec<_> = number_regex.find_iter(line).map(|m| (m.as_str().parse::<u32>().unwrap(), m.range())).collect();
        numbers.push(line_numbers);

        let line_symbols: Vec<_> = symbol_regex.find_iter(line).map(|m| (m.as_str().chars().next().unwrap(), m.range().start)).collect();
        symbols.push(line_symbols);
    }

    let mut sum = 0;
    for (line, line_numbers) in numbers.iter().enumerate() {
        for (number, range) in line_numbers {
            let extended_range = range.start.saturating_sub(1) ..= range.end;

            for line in line.saturating_sub(1) ..= (line + 1) {
                if let Some(symbols) = symbols.get(line) {
                    if symbols.iter().any(|s| extended_range.contains(&s.1)) {
                        sum += number;
                        break;
                    }
                }
            }
        }
    }

    sum
}

fn part2(input: &str) -> u32 {
    0
}

#[cfg(test)]
const INPUT: &str = "\
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
";

#[test]
fn test1() {
    assert_eq!(part1(INPUT), 4361);
}

#[test]
fn test2() {
    assert_eq!(part2(INPUT), 0);
}