fn find_max_sum(input: &str, n: usize) -> u32 {
    let sums = input
        .split("\n\n")
        .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum());

    if n == 1 {
        return sums.max().unwrap();
    }

    let mut sum_vec: Vec<_> = sums.collect();
    sum_vec.sort_unstable_by(|a, b| b.cmp(a));
    sum_vec.iter().take(n).sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(find_max_sum(input, 1))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(find_max_sum(input, 3))
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
