fn main() {
    let input = std::fs::read_to_string("./input/day2.txt").unwrap();

    
    println!("Part 1 solution: {}", part1(&input));
    println!("Part 2 solution: {}", part2(&input));
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;

    'game: for line in input.lines() {
        let (game, sets) = line.split_once(": ").unwrap();
        let game = game.split_once(" ").unwrap().1.parse::<u32>().unwrap();
        let sets = sets
            .split("; ")
            .map(|set| set
                .split(", ")
                .map(|entry| entry.split_once(" ").unwrap())
                .map(|(n, color)| (n.parse::<u32>().unwrap(), color))
                .collect::<Vec<_>>())
            .collect::<Vec<_>>();

        for set in sets {
            for (count, color) in set {
                match color {
                    "red" => if count > 12 { continue 'game; }
                    "green" => if count > 13 { continue 'game; }
                    "blue" => if count > 14 { continue 'game; }
                    _ => unreachable!()
                }
            }
        }

        sum += game;
    }

    sum
}


fn part2(input: &str) -> u32 {
    let mut sum = 0;

    'game: for line in input.lines() {
        let (game, sets) = line.split_once(": ").unwrap();
        let game = game.split_once(" ").unwrap().1.parse::<u32>().unwrap();
        let sets = sets
            .split("; ")
            .map(|set| set
                .split(", ")
                .map(|entry| entry.split_once(" ").unwrap())
                .map(|(n, color)| (n.parse::<u32>().unwrap(), color))
                .collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut min = (0, 0, 0);

        for set in sets {
            for (count, color) in set {
                match color {
                    "red" => min.0 = min.0.max(count),
                    "green" => min.1 = min.1.max(count),
                    "blue" => min.2 = min.2.max(count),
                    _ => unreachable!()
                }
            }
        }

        sum += min.0 * min.1 * min.2;
    }

    sum
}

#[cfg(test)]
const INPUT: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

#[test]
fn test1() {
    assert_eq!(part1(INPUT), 8);
}

#[test]
fn test2() {
    assert_eq!(part2(INPUT), 2286);
}