use std::fs;

fn main() {
    // remember to change the input
    let file = fs::read_to_string("./inputs/day_04_input.txt").unwrap();
    println!("{}", part_1(&file));
    println!("{}", part_2(&file));
}

fn part_1(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();

    let mut sum = 0;
    for line in lines {
        let (left_pair, right_pair) = get_pairs(line);

        sum += get_points_part1(left_pair, right_pair);
    }

    sum
}

fn get_points_part1(left_pair: PairRange, right_pair: PairRange) -> u32 {
    (left_pair.pair_fully_contains(&right_pair) || right_pair.pair_fully_contains(&left_pair))
        as u32
}

fn part_2(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();

    let mut sum = 0;
    for line in lines {
        let (left_pair, right_pair) = get_pairs(line);

        sum += get_points_part2(left_pair, right_pair);
    }

    sum
}

fn get_points_part2(left_pair: PairRange, right_pair: PairRange) -> u32 {
    left_pair.some_overlap(&right_pair) as u32
}

fn get_pairs(line: &str) -> (PairRange, PairRange) {
    // this is all parsing
    // split on ,
    // then split numbers on -
    // turn them into PairRanges
    // so I think it makes the most sense to be 1 fn.
    // no reason to separate the splitting on , and -
    // or even parsing ints and turning into structs

    let ranges: Vec<&str> = line.split(',').collect();

    let left_range: Vec<&str> = ranges[0].split('-').collect();
    let right_range: Vec<&str> = ranges[1].split('-').collect();

    let left_pair = PairRange::new(
        left_range[0].parse().unwrap(),
        left_range[1].parse().unwrap(),
    );
    let right_pair = PairRange::new(
        right_range[0].parse().unwrap(),
        right_range[1].parse().unwrap(),
    );

    (left_pair, right_pair)
}

struct PairRange {
    start: u32,
    end: u32,
}

impl PairRange {
    fn new(start: u32, end: u32) -> Self {
        PairRange { start, end }
    }

    fn pair_fully_contains(&self, other: &PairRange) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn pair_less_than(&self, other: &PairRange) -> bool {
        self.end < other.start
    }

    fn some_overlap(&self, other: &PairRange) -> bool {
        // is a pair less than another
        // less than is 1-3 < 4-6
        // max of a is < the min of b
        // 4-6, 1-3 also counts. (the second check is checking the other order)
        // if either is smaller, they don't overlap
        // so if neither pass, they overlap somewhat
        // given: 3-7, 4-8
        // 3-7 < 4-8 false
        // 4-8 < 3-7 false
        !(self.pair_less_than(other) || other.pair_less_than(self))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const DAY_4_BASIC_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part_1_works() {
        let result = part_1(&DAY_4_BASIC_INPUT);

        assert_eq!(2, result);
    }

    #[test]
    fn part_2_works() {
        let result = part_2(&DAY_4_BASIC_INPUT);

        assert_eq!(4, result);
    }
}
