fn main() {
    let input = std::fs::read_to_string("./input/day4.txt").unwrap();
    
    println!("Part 1 solution: {}", part1(parse(&input)));
    println!("Part 2 solution: {}", part2(parse(&input)));
}

struct Card {
    index: usize,
    winning: Vec<u32>,
    owned: Vec<u32>,
}

impl Card {
    fn matches(&self) -> usize {
        let mut count = 0;
        for owned in &self.owned {
            if self.winning.contains(&owned) {
                count += 1;
            }
        }
        count
    }
}

fn parse(input: &str) -> Vec<Card> {
    let mut cards = Vec::new();

    for line in input.lines() {
        let (card, numbers) = line.split_once(':').unwrap();
        let (winning, owned) = numbers.split_once('|').unwrap();

        let index = card[4..].trim().parse().unwrap();
        let winning = winning.split_whitespace().map(str::parse).map(Result::unwrap).collect();
        let owned = owned.split_whitespace().map(str::parse).map(Result::unwrap).collect();

        cards.push(Card {
            index,
            winning,
            owned
        });
    }

    cards
}

fn part1(input: Vec<Card>) -> u32 {
    let mut sum = 0;

    for card in input {
        sum +=  1 << card.matches() >> 1;
    }

    sum
}

fn part2(input: Vec<Card>) -> u32 {
    let mut count = vec![1; input.len()];

    for card in input {
        let base = card.index - 1;

        for i in 0 .. card.matches() {
            count[base + i + 1] += count[base];
        }
    }

    count.iter().sum()
}

#[cfg(test)]
const INPUT: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

#[test]
fn test1() {
    assert_eq!(part1(parse(INPUT)), 13);
}

#[test]
fn test2() {
    assert_eq!(part2(parse(INPUT)), 30);
}