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

type Input = Vec<(Hand, u32)>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Kind { HighCard, OnePair, TwoPair, ThreeKind, FullHouse, FourKind, FiveKind }

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.kind().cmp(&other.kind()).then(self.cards.cmp(&other.cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Hand {
    fn kind(&self) -> Kind {
        let mut count: HashMap<_, u32> = HashMap::with_capacity(self.cards.len());
        for card in self.cards {
            *count.entry(card).or_default() += 1;
        }

        let mut kind = Kind::HighCard;
        for count in count.into_values() {
            kind = match (count, kind) {
                (1, _) => kind,

                (2, Kind::ThreeKind) => Kind::FullHouse,
                (2, Kind::OnePair) => Kind::TwoPair,
                (2, Kind::HighCard) => Kind::OnePair,
                
                (3, Kind::OnePair) => Kind::FullHouse,
                (3, Kind::HighCard) => Kind::ThreeKind,
                
                (4, Kind::HighCard) => Kind::FourKind,
                (5, Kind::HighCard) => Kind::FiveKind,

                _ => kind
            }
        }

        kind
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card { Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace }

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!()
        }
    }
}


fn parse(input: &str) -> Input {
    let mut result = Input::new();

    for line in input.lines() {
        let (hand, bid) = line.split_once(' ').unwrap();
        let hand = Hand { cards: hand.chars().map(Card::from).collect::<Vec<_>>().try_into().unwrap() };
        let bid = bid.parse().unwrap();
        result.push((hand, bid));
    }

    result
}


fn part1(mut input: Input) -> usize {
    input.sort_by_key(|e| e.0);
    input.iter().enumerate().map(|(i, &(_, bid))| (i + 1) * bid as usize).sum()
}

fn part2(input: Input) -> u32 {
    let mut result = 0;
    result
}