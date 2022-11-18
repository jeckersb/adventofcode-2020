use nom::{
    character::complete::{alpha1, anychar, char, newline, space1, u8},
    combinator::map,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

struct Policy<'a> {
    range: std::ops::RangeInclusive<usize>,
    letter: char,
    password: &'a str,
}

impl<'a> Policy<'a> {
    fn parse(input: &'a str) -> IResult<&str, Self> {
        map(
            tuple((
                u8,
                char('-'),
                u8,
                space1,
                anychar,
                char(':'),
                space1,
                alpha1,
            )),
            |(start, _dash, end, _space, letter, _colon, _space2, password)| Self {
                letter,
                password,
                range: start.into()..=end.into(),
            },
        )(input)
    }

    fn valid(&self) -> bool {
        self.range
            .contains(&self.password.chars().filter(|&c| c == self.letter).count())
    }

    fn valid2(&self) -> bool {
        let pos1 = self.range.start() - 1;
        let pos2 = self.range.end() - 1;

        let chars = self.password.chars().collect::<Vec<_>>();
        (chars[pos1] == self.letter) != (chars[pos2] == self.letter)
    }
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> u32 {
    match separated_list1(newline, Policy::parse)(input) {
        Ok((_, policies)) => policies
            .into_iter()
            .filter(Policy::valid)
            .count()
            .try_into()
            .unwrap(),
        Err(_) => panic!("Unable to parse policies"),
    }
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &str) -> u32 {
    match separated_list1(newline, Policy::parse)(input) {
        Ok((_, policies)) => policies
            .into_iter()
            .filter(Policy::valid2)
            .count()
            .try_into()
            .unwrap(),
        Err(_) => panic!("Unable to parse policies"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(
            solve_part1(
                "1-3 a: abcde\n\
		 1-3 b: cdefg\n\
		 2-9 c: ccccccccc"
            ),
            2
        );
    }

    #[test]
    fn examples_part2() {
        assert_eq!(
            solve_part2(
                "1-3 a: abcde\n\
		 1-3 b: cdefg\n\
		 2-9 c: ccccccccc"
            ),
            1
        );
    }
}
