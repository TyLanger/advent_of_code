use std::{fs, fmt::Display,};

fn main() {
    let input = fs::read_to_string("./inputs/day_20_input.txt").unwrap();

    println!("{}", part_1(&input));
}

fn part_1(input: &str) -> i32 {

    // I want to have the array of numbers
    // and an array of pointers
    // as the numbers move around, the pointers point to where they now are
    // so I can still move them in order.

    // [1,2,3,4]
    //  | | | |
    // [1,2,3,4]
    // swap 0, 1
    // [1,2,3,4]
    //   X | |
    // [2,1,3,4]

    // numbers has the values
    // pointers has the indicies
    // loop over pointers
    // when I do a swap, update the pointers array

    // numbers aren't unique. 3626 / 5000 set / vec

    // version 2:
    // Just brute force it
    // save the value and original order together
    // each time just search for the correct item

    let mut values = Vec::new();


    for (i, line) in input.lines().enumerate() {
        let num = line.trim().parse::<i32>().unwrap();
        
        let val = Node {
            og_order: i,
            value: num,
        };

        values.push(val);
        
    }
    // println!("Values: {:?}", values);
    // for v in &values {
    //     print!("{:}", v.to_string());
    // }
    // println!();


    let length = values.len();
    for i in 0..length {

        // find the node with the right order
        let mut current_index = 0;
        let mut current_value = 0;
        for (j, node) in values.iter().enumerate() {
            if node.og_order == i {
                current_index = j;
                current_value = node.value;
                break;
            }
        }

        if current_value > 0 {
            for s in 0..current_value {
                let s = s as usize;
                let start = (current_index + s) % length;
                let end = (current_index + s + 1) % length;
                values.swap(start, end);
            }
        } else {
            let abs_value = current_value.abs();
            for s in 0..abs_value {
                let s = s as usize;
                let start = sub_mod(current_index, s, length);
                let end = sub_mod(current_index, s + 1, length);
                values.swap(start, end);
            }
        }
        


    }
    
    // for v in &values {
    //     print!("{:}", v.to_string());
    // }
    // println!();

    let mut start_index = 0;
    for i in 0..values.len() {
        if values[i].value == 0 {
            start_index = i;
            break;
        }
    }
    let first = values[add_mod(start_index, 1000, length)];
    let second = values[add_mod(start_index, 2000, length)];
    let third = values[add_mod(start_index, 3000, length)];

    println!("first: {} second: {} third: {}", first.value, second.value, third.value);

    first.value + second.value + third.value
}

#[derive(Debug, Clone, Copy)]
struct Node {
    og_order: usize,
    value: i32,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]: {} ", self.og_order, self.value)
    }
}

fn add_one_mod(index: usize, len: usize) -> usize {
    (index + 1) % len
}

fn sub_one_mod(index: usize, len: usize) -> usize {
    (index + len - 1) % len
}

fn add_mod(index: usize, add: usize, len: usize) -> usize {
    (index + add) % len
}

fn sub_mod(index: usize, sub: usize, len: usize) -> usize {
    let sub = sub % len;
    (index + len - sub) % len
}

#[cfg(test)]
mod tests {

    use super::*;

    const BASIC_INPUT_DAY_20: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn part_1_works() {
        assert_eq!(3, part_1(&BASIC_INPUT_DAY_20));
    }

    #[test]
    fn wrap_helpers() {
        let len = 7;

        assert_eq!(5, add_one_mod(4, len));
        assert_eq!(0, add_one_mod(6, len));

        assert_eq!(4, sub_one_mod(5, len));
        assert_eq!(6, sub_one_mod(0, len));
    }
}
