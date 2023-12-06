#[cfg(test)]
const INPUT: &str = "\
Time:      7  15   30
Distance:  9  40  200
";

#[test]
fn test1() {
    assert_eq!(part1(parse(INPUT)), 288);
}

#[test]
fn test2() {
    assert_eq!(part2(parse(INPUT)), 71503);
}


fn main() {
    let input = std::fs::read_to_string("./input/day6.txt").unwrap();
    
    println!("Part 1 solution: {}", part1(parse(&input)));
    println!("Part 2 solution: {}", part2(parse(&input)));
}


struct Input {
    races: Vec<Race>
}

struct Race {
    time: usize,
    distance: usize,
}


fn parse(input: &str) -> Input {
    let mut lines = input.lines();

    let times: Vec<_> = lines.next().unwrap().split_whitespace().skip(1).map(str::parse).map(Result::unwrap).collect();
    let distances: Vec<_> = lines.next().unwrap().split_whitespace().skip(1).map(str::parse).map(Result::unwrap).collect();

    let races = times.iter().zip(distances.iter()).map(|(&time, &distance)| Race { time, distance }).collect();

    Input { races }
}


fn part1(input: Input) -> u32 {
    let mut total = 1;

    for race in input.races {
        let mut solutions = 0;
        for held in 0..race.time {
            let distance = (race.time - held) * held;
            if distance > race.distance {
                solutions += 1;
            }
        }

        total *= solutions;
    }

    total
}

fn part2(input: Input) -> u32 {
    let race = Race {
        time: input.races.iter().map(|r| &r.time).map(ToString::to_string).collect::<String>().parse().unwrap(),
        distance: input.races.iter().map(|r| &r.distance).map(ToString::to_string).collect::<String>().parse().unwrap(),
    };

    let mut solutions = 0;
    for held in 0..race.time {
        let distance = (race.time - held) * held;
        if distance > race.distance {
            solutions += 1;
        }
    }

    solutions
}