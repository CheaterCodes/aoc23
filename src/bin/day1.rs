fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./input/day1.txt")?;

    println!("Part 1 solution: {}", part1(&input));
    println!("Part 2 solution: {}", part2(&input));

    Ok(())
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let first = line.chars().find(char::is_ascii_digit).unwrap();
        let last = line.chars().rev().find(char::is_ascii_digit).unwrap();

        let value = first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap();

        sum += value;
    }

    sum
}

fn part2(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let mut first = None;
        let mut last = None;

        let words = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

        
        for i in 0..line.len() {
            let from_digit = line[i..].chars().next().unwrap().to_digit(10);
            let from_word = words.iter().enumerate().find(|w| line[i..].starts_with(w.1)).map(|w| w.0 as u32 + 1);
            let Some(value) = from_digit.or(from_word) else { continue; };

            last = Some(value);
            if first.is_none() {
                first = Some(value);
            }
        }

        sum += first.unwrap() * 10 + last.unwrap();
    }

    sum
}

#[cfg(test)]
const TEST1_INPUT: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST1_INPUT), 142);
}

#[cfg(test)]
const TEST2_INPUT: &str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

#[test]
fn test_part2() {
    assert_eq!(part2(TEST2_INPUT), 281);
}