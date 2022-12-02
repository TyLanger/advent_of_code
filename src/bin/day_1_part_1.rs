use std::fs;

// run with cargo run --bin day_1_part_1
fn main() {
    let file = fs::read_to_string("./inputs/day_01_input.txt").unwrap();
    println!("{}", get_most_calories(&file));
    println!("{}", get_most_calories_alt(&file));
    println!("{}", get_top_three_total(&file));
    println!("{}", get_top_three_total_alt(&file));
    println!("{}", get_top_total(&file, 1));
    println!("{}", get_top_total(&file, 3));
}

fn get_most_calories(input: &str) -> String {
    let most: u32 = input
        .split("\r\n\r\n")
        // .inspect(|split| println!("After split {:?}", split))
        .map(|elf| {
            elf.lines()
                // .inspect(|x| println!("before map {}", x))
                .filter_map(|item| item.parse::<u32>().ok())
                // .filter_map(Result::ok)
                .sum::<u32>()
        })
        .max()
        .unwrap();

    most.to_string()
}

fn get_most_calories_alt(input: &str) -> u32 {
    // lines will give
    // "1000"
    // "2000"
    // "3000"
    // ""

    // split \r\n\r\n will give
    // "1000\n2000\n3000"
    // "4000"

    // split \n\n doesn't work on windows
    // seems lines would be a more general solution

    let lines: Vec<&str> = input
        .lines()
        // .inspect(|split| println!("after split: {:?}", split))
        .collect();

    let mut sum = 0;
    let mut biggest = 0;
    for item in lines {
        if let Ok(i) = item.parse::<u32>() {
            sum += i;
            if sum > biggest {
                biggest = sum;
            }
        } else {
            sum = 0;
        }
    }

    biggest
}

fn get_top_three_total(input: &str) -> u32 {
    let mut totals: Vec<u32> = input
        .split("\r\n\r\n")
        .map(|elf| {
            elf.lines()
                .map(|item| item.parse::<u32>())
                .filter_map(Result::ok)
                .sum::<u32>()
        })
        .collect();

    totals.sort_by(|a, b| b.cmp(a));
    let sum = totals.iter().take(3).sum();

    sum
}

fn get_each_elf(input: &str) -> Vec<u32> {
    let lines: Vec<&str> = input.lines().collect();

    let mut elves: Vec<u32> = Vec::new();
    let mut sum = 0;
    for item in lines {
        if let Ok(i) = item.parse::<u32>() {
            sum += i;
        } else {
            // push once you reach a ""
            // that's the end of one elf and the start of another
            elves.push(sum);
            sum = 0;
        }
    }
    // push the last element
    elves.push(sum);

    elves
}

fn get_top_three_total_alt(input: &str) -> u32 {
    let mut elves = get_each_elf(input);

    elves.sort_by(|a, b| b.cmp(a));
    elves.iter().take(3).sum()
}

fn get_top_total(input: &str, top_x: usize) -> u32 {
    let mut elves = get_each_elf(input);

    // elves.sort();
    // elves.reverse();

    elves.sort_by(|a, b| b.cmp(a));

    elves.iter().take(top_x).sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn basic_input() {
        let file = fs::read_to_string("./inputs/day_01_basic_input.txt").unwrap();
        assert_eq!("24000".to_string(), get_most_calories(&file));
    }

    #[test]
    fn top_3_total() {
        let file = fs::read_to_string("./inputs/day_01_basic_input.txt").unwrap();

        assert_eq!(45000, get_top_three_total(&file))
    }

    #[test]
    fn alternate_part_1_works() {
        let file = fs::read_to_string("./inputs/day_01_basic_input.txt").unwrap();
        assert_eq!(24000, get_most_calories_alt(&file));
    }

    #[test]
    fn get_each_elf_total() {
        let file = fs::read_to_string("./inputs/day_01_basic_input.txt").unwrap();
        let result = get_each_elf(&file);
        assert_eq!(vec![6000, 4000, 11000, 24000, 10000], result);
    }

    #[test]
    fn top_3_total_alt() {
        let file = fs::read_to_string("./inputs/day_01_basic_input.txt").unwrap();

        assert_eq!(45000, get_top_three_total_alt(&file))
    }

    #[test]
    fn top_x_1_and_3_work() {
        let file = fs::read_to_string("./inputs/day_01_basic_input.txt").unwrap();

        assert_eq!(24000, get_top_total(&file, 1));
        assert_eq!(45000, get_top_total(&file, 3));
    }
}
