pub fn part_one(input: &str) -> Option<u32> {
    input
    .split("\n\n")
    .map(|x: &str| -> u32 {
        x.split("\n").fold(0, |a, b| a+b.parse::<u32>().unwrap())
    })
    .max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut values = input
    .split("\n\n")
    .map(|x: &str| -> u32 {
        x.split("\n").fold(0, |a, b| a+b.parse::<u32>().unwrap())
    })
    .collect::<Vec<u32>>();
    values.sort_by(|a, b| b.cmp(a));
    Some(values[0..3].iter().fold(0, |a,b| a+b))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
