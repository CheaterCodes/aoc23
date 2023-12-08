use std::collections::HashMap;

#[cfg(test)]
const INPUT: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

#[test]
fn test1() {
    assert_eq!(part1(parse(INPUT)), 6440);
}

#[test]
fn test2() {
    assert_eq!(part2(parse(INPUT)), 0);
}


fn main() {
    let input = std::fs::read_to_string("./input/day7.txt").unwrap();
    
    println!("Part 1 solution: {}", part1(parse(&input)));
    println!("Part 2 solution: {}", part2(parse(&input)));
}

type Input = Vec<([char; 5], u32)>;

fn parse(input: &str) -> Input {
    let mut result = Input::new();

    for line in input.lines() {
        let (hand, bid) = line.split_once(' ').unwrap();
        let hand = hand.chars().collect::<Vec<_>>().try_into().unwrap();
        let bid = bid.parse().unwrap();
        result.push((hand, bid));
    }

    result
}

fn score_card(card: char) -> u8 {
    match card {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!()
    }
}

fn score_hand(hand: [char; 5]) -> u64 {
    let mut count: Vec<(char, u8)> = Vec::new();
    for card in hand {
        if let Some((_, count)) = count.iter_mut().find(|(c, _)| *c == card) {
            *count += 1;
        } else {
            count.push((card, 1));
        }
    }
    count.sort_by_key(|(_, count)| *count);
    count.reverse();

    let count: Vec<u8> = count.into_iter().map(|c| c.1).collect();
    
    let mut score = match count.as_slice() {
        [5] => 6,
        [4, 1] => 5,
        [3, 2] => 4,
        [3, 1, 1] => 3,
        [2, 2, 1] => 2,
        [2, 1, 1, 1] => 1,
        [1, 1, 1, 1, 1] => 0,
        a @ _ => unreachable!("Unexpected: {:?}", a)
    };

    score = score_card(hand[0]) as u64 + (score << 8);
    score = score_card(hand[1]) as u64 + (score << 8);
    score = score_card(hand[2]) as u64 + (score << 8);
    score = score_card(hand[3]) as u64 + (score << 8);
    score = score_card(hand[4]) as u64 + (score << 8);


    score
}

fn part1(mut input: Input) -> usize {
    input.sort_by_cached_key(|e| score_hand(e.0));
    input.iter().enumerate().map(|(i, &(_, bid))| (i + 1) * bid as usize).sum()
}


fn part2(input: Input) -> u32 {
    let mut result = 0;
    result
}