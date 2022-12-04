use std::fs;

fn main() {
    let file = fs::read_to_string("./inputs/day_04_input.txt").unwrap();
    println!("{}", part_1(&file));
    println!("{}", part_2(&file));
    

    // println!("{}", priority_total(&file));
    // println!("{}", badge_priority_total(&file));
    // println!("{}", part_1_alt(&file));
    // println!("{}", part_2_alt(&file));
}


fn part_1(input: &str) -> u32 {

    let lines: Vec<&str> = input.lines().collect();

    let mut sum = 0;
    // split into 2 pairs
    for line in lines {
        let ranges: Vec<&str> = line.split(',').collect();

        let left_range: Vec<&str> = ranges[0].split('-').collect();
        let right_range: Vec<&str> = ranges[1].split('-').collect();

        let left_pair = PairRange::new(left_range[0].parse().unwrap(), left_range[1].parse().unwrap());
        let right_pair = PairRange::new(right_range[0].parse().unwrap(), right_range[1].parse().unwrap());
    
        if left_pair.pair_fully_contains(&right_pair) || right_pair.pair_fully_contains(&left_pair) {
            sum += 1;
        }
    }

    sum
}

fn part_2(input: &str) -> u32 {

    let lines: Vec<&str> = input.lines().collect();

    let mut sum = 0;
    // split into 2 pairs
    for line in lines {
        let ranges: Vec<&str> = line.split(',').collect();

        let left_range: Vec<&str> = ranges[0].split('-').collect();
        let right_range: Vec<&str> = ranges[1].split('-').collect();

        let left_pair = PairRange::new(left_range[0].parse().unwrap(), left_range[1].parse().unwrap());
        let right_pair = PairRange::new(right_range[0].parse().unwrap(), right_range[1].parse().unwrap());
    
        if left_pair.pair_less_than(&right_pair) || right_pair.pair_less_than(&left_pair) {
        } else {
            sum += 1;
            
        }
    }

    sum
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