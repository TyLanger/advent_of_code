use std::{collections::HashSet, fs};

fn main() {
    // remember to change the input
    let file = fs::read_to_string("./inputs/day_06_input.txt").unwrap();
    println!("{}", part_1(&file));
    println!("{}", part_2(&file));
}

fn part_1(input: &str) -> usize {
    get_no_duplicates_index(input, 4)
}

fn part_2(input: &str) -> usize {
    get_no_duplicates_index(input, 14)
}

fn get_no_duplicates_index(input: &str, num_distinct: usize) -> usize {
    let mut v = vec![""; num_distinct];

    for (i, letter) in input.split("").enumerate().skip(1) {
        let remainder = i % num_distinct;

        v[remainder] = letter;

        if i > num_distinct && all_different_vec_set(&v) {
            return i;
        }
    }
    panic!("Shouldn't get here.");
}

fn _all_different_vec(v: &Vec<&str>) -> bool {
    for i in 0..(v.len() - 1) {
        for j in (i + 1)..v.len() {
            if v[i] == v[j] {
                return false;
            }
        }
    }
    true
}

fn all_different_vec_set(v: &Vec<&str>) -> bool {
    // this is probably better than the double loop
    // and easier to see what is going on I think
    let mut set = HashSet::new();
    for letter in v {
        let was_new = set.insert(letter);
        if !was_new {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT_1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const TEST_INPUT_2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const TEST_INPUT_3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const TEST_INPUT_4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const TEST_INPUT_5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn part_1_works() {
        let result = part_1(&TEST_INPUT_1);
        assert_eq!(7, result);

        let result = part_1(&TEST_INPUT_2);
        assert_eq!(5, result);

        let result = part_1(&TEST_INPUT_3);
        assert_eq!(6, result);

        let result = part_1(&TEST_INPUT_4);
        assert_eq!(10, result);

        let result = part_1(&TEST_INPUT_5);
        assert_eq!(11, result);
    }

    #[test]
    fn part_2_works() {
        let result = part_2(&TEST_INPUT_1);
        assert_eq!(19, result);

        let result = part_2(&TEST_INPUT_2);
        assert_eq!(23, result);

        let result = part_2(&TEST_INPUT_3);
        assert_eq!(23, result);

        let result = part_2(&TEST_INPUT_4);
        assert_eq!(29, result);

        let result = part_2(&TEST_INPUT_5);
        assert_eq!(26, result);
    }
}
