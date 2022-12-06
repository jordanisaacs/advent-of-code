const WIN: u32 = 6;
const DRAW: u32 = 3;

#[derive(PartialEq, Eq)]
enum Action {
    Rock,
    Paper,
    Scissors,
}

enum Response {
    Win,
    Draw,
    Lose,
}

impl Response {
    fn parse_resp(s: char) -> Self {
        use Response::*;
        match s {
            'X' => Lose,
            'Y' => Draw,
            'Z' => Win,
            _ => panic!("Not valid response strategy"),
        }
    }

    fn get_score(self, opp: &Action) -> u32 {
        use Response::*;
        match (self, opp) {
            (Win, x) => x.map_win().score_action() + WIN,
            (Draw, x) => x.score_action() + DRAW,
            (Lose, x) => x.map_lose().score_action(),
        }
    }
}

impl Action {
    fn parse_opp(s: char) -> Self {
        use Action::*;
        match s {
            'A' => Rock,
            'B' => Paper,
            'C' => Scissors,
            _ => panic!("Not valid opponent strategy"),
        }
    }

    fn map_win(&self) -> Self {
        use Action::*;
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    fn map_lose(&self) -> Self {
        use Action::*;
        match self {
            Paper => Rock,
            Scissors => Paper,
            Rock => Scissors,
        }
    }

    fn parse_resp(s: char) -> Self {
        use Action::*;
        match s {
            'X' => Rock,
            'Y' => Paper,
            'Z' => Scissors,
            _ => panic!("Not valid response strategy"),
        }
    }

    fn score_action(&self) -> u32 {
        use Action::*;
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn get_score(&self, opponent: &Action) -> u32 {
        let shape_score = self.score_action();
        if self == opponent {
            return shape_score + DRAW;
        }

        use Action::*;
        match (self, opponent) {
            (Scissors, Paper) => shape_score + WIN,
            (Paper, Rock) => shape_score + WIN,
            (Rock, Scissors) => shape_score + WIN,
            (_, _) => shape_score,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let score = input
        .lines()
        .map(|l| {
            let mut iter = l.chars();
            let o = Action::parse_opp(iter.nth(0).unwrap());
            let r = Action::parse_resp(iter.nth(1).unwrap());
            r.get_score(&o)
        })
        .sum::<u32>();

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let score = input
        .lines()
        .map(|l| {
            let mut iter = l.chars();
            let o = Action::parse_opp(iter.nth(0).unwrap());
            let r = Response::parse_resp(iter.nth(1).unwrap());
            r.get_score(&o)
        })
        .sum::<u32>();
    Some(score)
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
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
