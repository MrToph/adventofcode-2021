pub fn run(list: &[&str]) -> Option<usize> {
    let mut counter = 0;

    Some(counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: [&str; 6] = [
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];
    // const PUZZLE_INPUT: [usize; 2000] = [
    //     10751, 10755, 10756, 10753,
    // ];

    #[test]
    fn part1_works_for_sample_input() {
        assert_eq!(run(&SAMPLE_INPUT).unwrap(), 150);
    }

    // #[test]
    // fn part1_works_for_puzzle_input() {
    //     assert_eq!(run(&PUZZLE_INPUT).unwrap(), 1766);
    // }
}
