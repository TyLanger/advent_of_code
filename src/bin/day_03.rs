use std::{collections::HashSet, fs};

fn main() {
    let file = fs::read_to_string("./inputs/day_03_input.txt").unwrap();
    println!("{}", priority_total(&file));
    println!("{}", badge_priority_total(&file));
    println!("{}", part_1_alt(&file));
    println!("{}", part_2_alt(&file));
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

fn badge_priority_total(input: &str) -> u32 {
    // split into sets of 3 lines
    // find the char that appears in all 3 lines
    // get the priority of that letter
    // sum all priorities

    let lines = input.lines();
    let counter: Vec<&str> = lines.collect();

    let mut sum = 0;
    for i in 0..(counter.len() / 3) {
        let line_a = counter[i * 3];
        let line_b = counter[i * 3 + 1];
        let line_c = counter[i * 3 + 2];

        // I feel like I should be able to do it with next
        // let line_a = lines.next();
        // let line_b = lines.next();
        // let line_c = lines.next();

        let result = get_triplicate_letter(line_a, line_b, line_c);
        sum += result;
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
    "0"
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
        if set_a.contains(letter_c) && set_b.contains(letter_c) {
            let char_vec: Vec<char> = letter_c.chars().collect();
            return letter_to_priority(char_vec[0]);
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

fn get_letter_in_each<'a>(inputs: &'a [&'a str]) -> Option<&'a str> {
    // alternate back and forth between the sets
    let mut set_a = HashSet::new();
    let mut set_b = HashSet::new();

    // fill set a with every element of the first input
    for letter_a in inputs[0].split("").filter(|x| !x.is_empty()) {
        set_a.insert(letter_a);
    }

    // then go to the next input
    // fill set_b with items in input[1] that set_a contains
    // now set_b contains the intersection of the first 2 inputs
    // clear set_a
    // now fill set_a with items in input[2] that set_b contains
    // set_a now contains the itersection of the first 3 inputs
    // keep alternating sets. Check against the old one and store the new intersection in the new one.
    // with good inputs, the last used set contains 1 item

    let mut use_set_a = true;
    for input in inputs.iter().skip(1) {
        if use_set_a {
            set_b.clear();
            for letter in input.split("").filter(|x| !x.is_empty()) {
                if set_a.contains(letter) {
                    set_b.insert(letter);
                }
            }
        } else {
            set_a.clear();
            for letter in input.split("").filter(|x| !x.is_empty()) {
                if set_b.contains(letter) {
                    set_a.insert(letter);
                }
            }
        }
        use_set_a = !use_set_a;
    }

    if use_set_a {
        set_a.drain().next()
        // if let Some(i) = set_a.drain().next() {
        //     return Some(i);
        // }
        // for i in set_a.drain() {
        //     return Some(i);
        // }
    } else {
        set_b.drain().next()

        // for i in set_b.drain() {
        //     return Some(i);
        // }
    }
}

fn part_1_alt(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();

    let mut sum = 0;
    for line in lines {
        let rucksack = line.split_at(line.len() / 2);

        let vec = vec![rucksack.0, rucksack.1];
        let letter = get_letter_in_each(&vec);
        if let Some(letter) = letter {
            let char_vec: Vec<char> = letter.chars().collect();
            let prio = letter_to_priority(char_vec[0]);
            sum += prio;
        }
    }

    sum
}

fn part_2_alt(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();

    let mut sum = 0;
    for i in 0..(lines.len() / 3) {
        let a = lines[3 * i];
        let b = lines[3 * i + 1];
        let c = lines[3 * i + 2];
        let vec = vec![a, b, c];

        let letter = get_letter_in_each(&vec);

        if let Some(letter) = letter {
            let char_vec: Vec<char> = letter.chars().collect();
            let prio = letter_to_priority(char_vec[0]);
            sum += prio;
        }
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

    #[test]
    fn rolling_sets() {
        // works for 3 (odd)
        let inputs = ["abc", "cbed", "bio"];
        let same = get_letter_in_each(&inputs);

        assert_eq!("b", same.unwrap());

        // works for 4 (even)
        let inputs = ["abc", "cbed", "bio", "abe"];
        let same = get_letter_in_each(&inputs);

        assert_eq!("b", same.unwrap());

        // works for 4 (even)
        let inputs = ["abc", "cbed", "bio", "abe", "zzzx"];
        let same = get_letter_in_each(&inputs);

        assert_eq!(None, same);
    }

    #[test]
    fn refactor_part_1() {
        let result = priority_total(DAY_3_BASIC_INPUT);
        let refactor = part_1_alt(DAY_3_BASIC_INPUT);

        assert_eq!(refactor, result);
    }

    #[test]
    fn refactor_part_2() {
        let result = badge_priority_total(DAY_3_BASIC_INPUT);
        let refactor = part_2_alt(DAY_3_BASIC_INPUT);

        assert_eq!(refactor, result);
    }
}
