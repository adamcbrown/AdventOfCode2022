use pico_args::Error;

enum Move {
    Rock, Paper, Scissors
}

fn compute_score_1(opp: char, mine: char) -> u32 {
    match mine {
        'X' => 1 + match opp {
            'A' => 3,
            'B' => 0,
            'C' => 6,
            _ => unreachable!(),
        },
        'Y' => 2 + match opp {
            'A' => 6,
            'B' => 3,
            'C' => 0,
            _ => unreachable!(),
        },
        'Z' => 3 + match opp {
            'A' => 0,
            'B' =>  6,
            'C' =>  3,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn compute_score_2(opp: char, result: char) -> u32 {
    match result {
        'X' => 0 + match opp {
            'A' => 3,
            'B' => 1,
            'C' => 2,
            _ => unreachable!(),
        },
        'Y' => 3 + match opp {
            'A' => 1,
            'B' => 2,
            'C' => 3,
            _ => unreachable!(),
        },
        'Z' => 6 + match opp {
            'A' => 2,
            'B' => 3,
            'C' => 1,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}


pub fn part_one(input: &str) -> Option<u32> {
    Some(input
        .split("\n")
        .map(|round| compute_score_1(round.chars().nth(0).unwrap(), round.chars().nth(2).unwrap()))
        .fold(0, |a, b| a+b))
    
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input
        .split("\n")
        .map(|round| compute_score_2(round.chars().nth(0).unwrap(), round.chars().nth(2).unwrap()))
        .fold(0, |a, b| a+b))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
