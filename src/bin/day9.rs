#[cfg(test)]
const INPUT: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

#[test]
fn test1() {
    assert_eq!(part1(parse(INPUT)), 114);
}

#[test]
fn test2() {
    assert_eq!(part2(parse(INPUT)), 2);
}


fn main() {
    let input = std::fs::read_to_string("./input/day9.txt").unwrap();
    
    println!("Part 1 solution: {}", part1(parse(&input)));
    println!("Part 2 solution: {}", part2(parse(&input)));
}


type Input = Vec<Vec<i32>>;

fn parse(input: &str) -> Input {
    input.lines().map(|line| line.split_whitespace().map(str::parse).map(Result::unwrap).collect()).collect()
}


fn part1(input: Input) -> i32 {
    let mut sum = 0;

    for history in input {
        let mut derivatives = vec![history];
        while derivatives.last().is_some_and(|x| x.iter().any(|&v| v != 0)) {
            derivatives.push(derivatives.last().unwrap().windows(2).map(|w| w[1] - w[0]).collect())
        }

        sum += derivatives.iter().filter_map(|d| d.last().copied()).sum::<i32>();
    }

    sum
}

fn part2(input: Input) -> i32 {
    let mut sum = 0;

    for history in input {
        let mut derivatives = vec![history];
        while derivatives.last().is_some_and(|x| x.iter().any(|&v| v != 0)) {
            derivatives.push(derivatives.last().unwrap().windows(2).map(|w| w[1] - w[0]).collect())
        }

        sum += derivatives.iter().rev().filter_map(|d| d.first().copied()).fold(0, |a, b| b - a);
    }

    sum
}