static INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn score(&self) -> usize {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    pub fn new(your_choice: Hand, opponent_choice: Hand) -> Self {
        match (your_choice, opponent_choice) {
            (Hand::Rock, Hand::Rock) => Self::Draw,
            (Hand::Rock, Hand::Paper) => Self::Loss,
            (Hand::Rock, Hand::Scissors) => Self::Win,
            (Hand::Paper, Hand::Rock) => Self::Win,
            (Hand::Paper, Hand::Paper) => Self::Draw,
            (Hand::Paper, Hand::Scissors) => Self::Loss,
            (Hand::Scissors, Hand::Rock) => Self::Loss,
            (Hand::Scissors, Hand::Paper) => Self::Win,
            (Hand::Scissors, Hand::Scissors) => Self::Draw,
        }
    }

    pub fn score(&self) -> usize {
        match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

impl From<char> for Outcome {
    fn from(value: char) -> Self {
        match value {
            'X' => Self::Loss,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            c => panic!("unknown outcome: {c}"),
        }
    }
}

impl TryFrom<char> for Hand {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' | 'A' => Ok(Self::Rock),
            'Y' | 'B' => Ok(Self::Paper),
            'Z' | 'C' => Ok(Self::Scissors),
            c => Err(c),
        }
    }
}

#[derive(Debug)]
struct Round {
    your_choice: Hand,
    opponent_choice: Hand,
}

impl Round {
    fn score(&self) -> usize {
        Outcome::new(self.your_choice, self.opponent_choice).score() + self.your_choice.score()
    }
}

impl TryFrom<&str> for Round {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut chars = value.chars();
        let opponent_choice = Hand::try_from(chars.next().unwrap()).unwrap();
        chars.next().unwrap();
        let your_choice = Hand::try_from(chars.next().unwrap()).unwrap();
        Ok(Self {
            your_choice,
            opponent_choice,
        })
    }
}

fn strategy_guide_score(guide: &str) -> usize {
    guide
        .lines()
        .map(|line| Round::try_from(line).unwrap().score())
        .sum()
}

pub fn part1() -> usize {
    strategy_guide_score(INPUT)
}

fn elf_instruction_score(guide: &str) -> usize {
    guide
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let opponent_choice = Hand::try_from(chars.next().unwrap()).unwrap();
            chars.next();
            let outcome = Outcome::from(chars.next().unwrap());

            let your_choice = match (opponent_choice, outcome) {
                (Hand::Rock, Outcome::Loss) => Hand::Scissors,
                (Hand::Rock, Outcome::Draw) => Hand::Rock,
                (Hand::Rock, Outcome::Win) => Hand::Paper,
                (Hand::Paper, Outcome::Loss) => Hand::Rock,
                (Hand::Paper, Outcome::Draw) => Hand::Paper,
                (Hand::Paper, Outcome::Win) => Hand::Scissors,
                (Hand::Scissors, Outcome::Loss) => Hand::Paper,
                (Hand::Scissors, Outcome::Draw) => Hand::Scissors,
                (Hand::Scissors, Outcome::Win) => Hand::Rock,
            };

            Round {
                your_choice,
                opponent_choice,
            }
            .score()
        })
        .sum()
}

pub fn part2() -> usize {
    elf_instruction_score(INPUT)
}

mod tests {
    use super::*;

    #[test]
    fn example() {
        static EXAMPLE_INPUT: &str = "A Y
B X
C Z";
        assert_eq!(strategy_guide_score(EXAMPLE_INPUT), 15);
    }

    #[test]
    fn example2() {
        static EXAMPLE_INPUT: &str = "A Y
B X
C Z";
        assert_eq!(elf_instruction_score(EXAMPLE_INPUT), 12);
    }
}
