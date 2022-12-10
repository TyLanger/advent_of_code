use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/day_num_input.txt").unwrap();

    println!("{}", part_1(&input));
}

fn part_1(input: &str) -> u32 {
    99
}

#[cfg(test)]
mod tests {

    use super::*;

    const BASIC_INPUT_DAY_NUM: &str = "";

    #[test]
    fn part_1_works() {
        assert_eq!(0, part_1(&BASIC_INPUT_DAY_NUM));
    }
}
