use std::{collections::HashSet, fs};

fn main() {
    let file = fs::read_to_string("./inputs/day_03_input.txt").unwrap();
    println!("{}", priority_total(&file));
    println!("{}", badge_priority_total(&file));
    // println!("{}", score_from_strategy_guide_part_2(&file));
}

fn priority_total(input: &str) -> u32 {
    // split the rucksacks
    let rucksacks = input.lines();

    let mut sum = 0;
    for ruck in rucksacks {
        // split the input in half
        // let halves = ruck.split_at(ruck.len()/2);

        let duplicate = get_duplicate_letter(ruck);

        let letter: Vec<char> = duplicate.chars().collect();
        // dbg!(letter.clone());
        // get the value of that char (cast to int?)
        let prio = letter_to_priority(letter[0]);

        sum += prio;
    }

    sum
}

fn get_duplicate_letter(input: &str) -> &str {
    let halves = input.split_at(input.len() / 2);

    let mut set = HashSet::new();

    let _first_half: Vec<bool> = halves
        .0
        .split("")
        .filter(|x| !x.is_empty())
        .map(|letter| set.insert(letter))
        .collect();

    // let dupe_letter: Vec<&str> = halves.1.split("").filter(|letter| set.contains(letter)).collect();

    // // should only be one
    // dupe_letter[0]

    for letter in halves.1.split("") {
        if set.contains(letter) {
            // dbg!(letter);
            return letter;
        }
    }
    dbg!("Not supposed to be here");
    return "0";
}

fn get_triplicate_letter(line_a: &str, line_b: &str, line_c: &str) -> u32 {
    let mut set_a = HashSet::new();
    let mut set_b = HashSet::new();

    for letter_a in line_a.split("").filter(|x| !x.is_empty()) {
        set_a.insert(letter_a);
    }

    for letter_b in line_b.split("").filter(|x| !x.is_empty()) {
        set_b.insert(letter_b);
    }

    for letter_c in line_c.split("").filter(|x| !x.is_empty()) {
        if set_a.contains(letter_c) {
            if set_b.contains(letter_c) {
                let char_vec: Vec<char> = letter_c.chars().collect();
                return letter_to_priority(char_vec[0]);
            }
        }
    }

    0
}

fn letter_to_priority(letter: char) -> u32 {
    if letter.is_lowercase() {
        letter as u32 - 96
    } else {
        letter as u32 - 38
    }
}

fn badge_priority_total(input: &str) -> u32 {
    // split into sets of 3 lines
    // find the char that appears in all 3 lines
    // get the priority of that letter
    // sum all priorities

    let lines = input.lines();
    let counter: Vec<&str> = lines.collect();

    let mut sum = 0;
    for i in 0..(counter.len() / 3) {
        let line_a = counter[i*3];
        let line_b = counter[i*3 + 1];
        let line_c = counter[i*3 + 2];

        // I feel like I should be able to do it with next
        // let line_a = lines.next();
        // let line_b = lines.next();
        // let line_c = lines.next();

        let result = get_triplicate_letter(line_a, line_b, line_c);
        sum += result;
    }

    sum
}

#[cfg(test)]
mod tests {

    use super::*;

    const DAY_3_BASIC_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part_1_works() {
        let result = priority_total(DAY_3_BASIC_INPUT);

        assert_eq!(157, result);
    }

    #[test]
    fn letter_to_priority_test() {
        // A is 65
        let priority = letter_to_priority('A');
        assert_eq!(priority, 27);

        // Z is 90
        let priority = letter_to_priority('Z');
        assert_eq!(priority, 52);

        // a is 97
        let priority = letter_to_priority('a');
        assert_eq!(priority, 1);

        // z is 122
        let priority = letter_to_priority('z');
        assert_eq!(priority, 26);
    }

    #[test]
    fn part_2_works() {
        let result = badge_priority_total(DAY_3_BASIC_INPUT);

        assert_eq!(70, result);
    }
}
