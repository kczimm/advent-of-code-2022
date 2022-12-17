use core::num::ParseIntError;

use crate::DOUBLE_NEWLINE;

static INPUT: &str = include_str!("input.txt");

pub fn part1() -> Calories {
    Elves::try_from(INPUT)
        .expect("parsing elves failed")
        .0
        .iter()
        .max()
        .expect("at least one elf")
        .calories
}

pub fn part2() -> Calories {
    let mut elves = Elves::try_from(INPUT).expect("parsing elves failed");
    elves.0.sort();
    elves
        .0
        .iter()
        .skip(elves.0.len() - 3)
        .map(|elf| elf.calories)
        .sum()
}

pub type Calories = usize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Elf {
    pub calories: Calories,
}

impl TryFrom<&str> for Elf {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value
            .lines()
            .try_fold(0, |acc, line| line.parse::<Calories>().map(|c| c + acc))
            .map(|calories| Self { calories })
    }
}

#[derive(Debug, PartialEq)]
pub struct Elves(Vec<Elf>);

impl TryFrom<&str> for Elves {
    type Error = ParseIntError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let elves: Result<Vec<_>, _> = value
            .split(DOUBLE_NEWLINE)
            .map(|v| Elf::try_from(v))
            .collect();
        Ok(Self(elves?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn elf_try_from() {
        assert_eq!(
            Elf::try_from(EXAMPLE_INPUT.split("\n\n").next().unwrap())
                .unwrap()
                .calories,
            6000
        );
    }

    #[test]
    fn elves_try_from() {
        assert_eq!(
            Elves::try_from(EXAMPLE_INPUT).unwrap(),
            Elves(vec![
                Elf { calories: 6000 },
                Elf { calories: 4000 },
                Elf { calories: 11_000 },
                Elf { calories: 24_000 },
                Elf { calories: 10_000 }
            ])
        )
    }
}
