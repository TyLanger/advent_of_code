use std::fs;

fn main() {
    // remember to change the input
    let file = fs::read_to_string("./inputs/day_05_input.txt").unwrap();
    println!("{}", part_1(&file));
    // println!("{}", part_2(&file));
}

fn part_1(input: &str) -> String {
    // get initial state
    // calculate all the moves
    // get the top of each stack
    // return the tops.

    let num_stacks = get_number_of_stacks(&input);
    let mut stacks = Vec::new();
    for _ in 0..num_stacks {
        let stack: Vec<String> = Vec::new();
        stacks.push(stack);
    }

    // create the starting stacks
    let start_stack_input: Vec<&str> = input.split('1').collect();

    setup_stacks(&mut stacks, start_stack_input[0]);
    // println!("{:?}", stacks);

    let instructions = input.split_once(&num_stacks.to_string());
    // split at the end of stack labels

    process_stacks(&mut stacks, instructions.unwrap().1);

    get_top_of_each_stack(stacks)
}

fn get_number_of_stacks(input: &str) -> u32 {
    // the line 1 2 3 ... 9
    // the last number is the number of stacks
    // I want this whole line

    // split on the first 1 to get the right line
    let number_split = input.split_once('1');
    // split into lines to get the whole line
    let lines: Vec<&str> = number_split.unwrap().1.lines().collect();
    // split into a vec of each number
    let all_nums: Vec<&str> = lines[0].split_whitespace().collect();
    // grab the last number (the number of stacks)
    all_nums.last().unwrap().parse().unwrap()
}

fn setup_stacks(stacks: &mut Vec<Vec<String>>, input: &str) {
    // need to start at the bottom
    let lines = input.lines();
    // split at 1 in the line below.
    // skip the line that is the few spaces in front of the 1
    for line in lines.rev().skip(1) {
        // let crates = line.split('[');
        // _, Z] ,M] ,P] 
        // len should be 4
        // _, N] ,C] 
        // len is 3 for 2 crates
        // _, A]     ,C]
        // for 2 crates with a gap in the middle
        // 5 spaces, len 3
        // ____,D]
        // 4 spaces for a gap

        // input has consistent spacing
        // check if there is a letter at position 1, 5, 9, etc.
        // if it exists, push it into that stack
        // let line_as_bytes = line.as_bytes();
        let individual_chars: Vec<&str> = line.split("").collect();
        let mut letter_offset = 2;
        for i in 0..stacks.len() {
            if letter_offset >= individual_chars.len() {
                break;
            }
            // check if a letter exists at the offset
            if !individual_chars[letter_offset].contains(" ") {
                stacks[i].push(individual_chars[letter_offset].to_string());
            }
            letter_offset += 4;
        }
    }
}

fn process_stacks(stacks: &mut Vec<Vec<String>>, input: &str) {

    // \n\r\n\r
    // move 1 from 2 to 1
    // move 3 from 1 to 3
    // ..

    let lines = input.lines();
    // skip the blank line
    for line in lines.skip(2) {
        let inst = Instruction::from_string(line);

        for _ in 0..inst.amount {
            let popped = stacks[inst.start - 1].pop();
            stacks[inst.dest - 1].push(popped.unwrap());
        }
    }
}

fn get_top_of_each_stack(stacks: Vec<Vec<String>>) -> String {
    let mut output = "".to_string();

    for stack in stacks {
        // breaks if a stack is empty
        // I don't know if I should skip an empty stack
        // or have a space or something
        // the requirements don't say
        // so I'll assume they won't be empty
        output = format!("{}{}", output, stack.last().unwrap());
    }

    output
}

struct Instruction {
    amount: u32,
    start: usize,
    dest: usize,
}

impl Instruction {
    fn from_string(line: &str) -> Self {
        let input: Vec<&str> = line.split_whitespace().collect();
        
        let amount = input[1].parse().unwrap();
        let start = input[3].parse().unwrap();
        let dest = input[5].parse().unwrap();

        Instruction {
            amount,
            start,
            dest,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const DAY_5_BASIC_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part_1_works() {
        let result = part_1(&DAY_5_BASIC_INPUT);

        assert_eq!("CMZ".to_string(), result);
    }

    #[test]
    fn test_number_of_stacks() {
        let amount = get_number_of_stacks(&DAY_5_BASIC_INPUT);

        assert_eq!(3, amount);
    }

    #[test]
    fn get_top_of_stacks() {
        let mut stacks = Vec::new();
        for _ in 0..3 {
            let stack: Vec<String> = Vec::new();
            stacks.push(stack);
        }
        stacks[0].push("Z".to_string());
        stacks[0].push("N".to_string());
        stacks[1].push("M".to_string());
        stacks[1].push("C".to_string());
        stacks[1].push("D".to_string());
        stacks[2].push("P".to_string());

        let output = get_top_of_each_stack(stacks);

        assert_eq!("NDP".to_string(), output);
    }

    #[test]
    fn construct_instruction_from_string_test() {
        let line = "move 1 from 2 to 3";

        let inst = Instruction::from_string(&line);

        assert_eq!(1, inst.amount);
        assert_eq!(2, inst.start);
        assert_eq!(3, inst.dest);
    }

    // #[test]
    // #[ignore = "not ready"]
    // fn part_2_works() {
    //     let result = part_2(&DAY_5_BASIC_INPUT);

    //     assert_eq!(4, result);
    // }
}
