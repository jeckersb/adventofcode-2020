use itertools::Itertools;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines().map(|c| c.parse().unwrap()).collect()
}

pub fn solve_kperm(input: &[u32], k: usize) -> u32 {
    input
        .iter()
        .permutations(k)
        .find_map(|perm| match perm.iter().copied().sum::<u32>() {
            2020 => Some(perm.into_iter().product()),
            _ => None,
        })
        .unwrap()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    solve_kperm(input, 2)
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    solve_kperm(input, 3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(
            solve_part1(&input_generator(
                "1721\n\
		 979\n\
		 366\n\
		 299\n\
		 675\n\
		 1456"
            )),
            514579
        );
    }

    #[test]
    fn examples_part2() {
        assert_eq!(
            solve_part2(&input_generator(
                "1721\n\
		 979\n\
		 366\n\
		 299\n\
		 675\n\
		 1456"
            )),
            241861950
        );
    }
}
