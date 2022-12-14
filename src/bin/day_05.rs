use std::fs;

fn main() {
    // remember to change the input
    let file = fs::read_to_string("./inputs/day_05_input.txt").unwrap();
    println!("{}", part_1(&file));
    println!("{}", part_2(&file));
}

fn part_1(input: &str) -> String {
    // get initial state
    // calculate all the moves
    // get the top of each stack
    // return the tops.

    let num_stacks = get_number_of_stacks(input);
    let mut stacks = Vec::new();
    for _ in 0..num_stacks {
        let stack: Vec<String> = Vec::new();
        stacks.push(stack);
    }

    // create the starting stacks
    setup_stacks(&mut stacks, input);

    // process the instructions on the stacks
    process_stacks(&mut stacks, input);

    get_top_of_each_stack(stacks)
}

fn part_2(input: &str) -> String {
    // get initial state
    // calculate all the moves
    // get the top of each stack
    // return the tops.

    let num_stacks = get_number_of_stacks(input);
    let mut stacks = Vec::new();
    for _ in 0..num_stacks {
        let stack: Vec<String> = Vec::new();
        stacks.push(stack);
    }

    // create the starting stacks
    setup_stacks(&mut stacks, input);

    // process the instructions on the stacks
    // only part that is different from part 1
    process_stacks_part_2(&mut stacks, input);

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
    // get only the line with the stack labels (other lines are the instructions)
    // split into a vec of each number
    let all_nums: Vec<&str> = lines[0].split_whitespace().collect();
    // grab the last number (the number of stacks)
    all_nums.last().unwrap().parse().unwrap()
}

fn setup_stacks(stacks: &mut [Vec<String>], input: &str) {
    // split at 1 in the line below.
    let start_stack_input: Vec<&str> = input.split('1').collect();
    let lines = start_stack_input[0].lines();

    // need to start at the bottom
    // skip the line that is the few spaces in front of the 1
    for line in lines.rev().skip(1) {
        // input has consistent spacing
        // check if there is a letter at position 1, 5, 9, etc.
        // if it exists, push it into that stack
        let individual_chars: Vec<&str> = line.split("").collect();
        // offset is 2, not 1 bc of the blank char at the start when splitting on ""
        let mut letter_offset = 2;
        for stack in stacks.iter_mut() {
            if letter_offset >= individual_chars.len() {
                break;
            }
            // check if a letter exists at the offset
            if !individual_chars[letter_offset].contains(' ') {
                stack.push(individual_chars[letter_offset].to_string());
            }
            letter_offset += 4;
        }
    }
}

fn process_stacks(stacks: &mut [Vec<String>], input: &str) {
    // _
    // \n\r\n\r
    // move 1 from 2 to 1
    // move 3 from 1 to 3
    // ..

    let instructions = input.split_once('1');
    // skip the blank lines
    // the bit after the stack labels (1  2  3  4, etc.)
    // and the gap line between the crates and instructions
    let lines = instructions.unwrap().1.lines().skip(2);

    // won't let me pass multiple to split_once
    // let instructions = input.split_once(&["\n\n", "\r\n\r\n"]);
    // let lines = instructions.unwrap().1.lines();

    // split won't work with multiple stringe either.
    // needs chars '-', ',', '+', etc.
    // let instructions = input.split(&["\n\n", "\r\n\r\n"]);

    for line in lines {
        let inst = Instruction::from_string(line);

        for _ in 0..inst.amount {
            let popped = stacks[inst.start - 1].pop();
            stacks[inst.dest - 1].push(popped.unwrap());
        }
    }
}

fn process_stacks_part_2(stacks: &mut [Vec<String>], input: &str) {
    // _
    // \n\r\n\r
    // move 1 from 2 to 1
    // move 3 from 1 to 3
    // ..

    // part 2:
    // can now move a whole stack at once
    // put into a temp stack

    let instructions = input.split_once('1');
    let lines = instructions.unwrap().1.lines().skip(2);

    for line in lines {
        let inst = Instruction::from_string(line);

        let mut temp_stack = Vec::new();
        for _ in 0..inst.amount {
            let popped = stacks[inst.start - 1].pop();
            temp_stack.push(popped);
        }

        for _ in 0..temp_stack.len() {
            let popped = temp_stack.pop();
            stacks[inst.dest - 1].push(popped.unwrap().unwrap());
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

    #[test]
    fn part_2_works() {
        let result = part_2(&DAY_5_BASIC_INPUT);

        assert_eq!("MCD".to_string(), result);
    }
}
