use std::{fmt::Display, fs};

fn main() {
    let input = fs::read_to_string("./inputs/day_20_input.txt").unwrap();

    // println!("{}", part_1(&input));
    // println!("{}", part_2(&input));
    println!("{}", part_2_smarter(&input));
}

fn part_1(input: &str) -> i64 {
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
        let num = line.trim().parse::<i64>().unwrap();

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

    println!(
        "first: {} second: {} third: {}",
        first.value, second.value, third.value
    );

    first.value + second.value + third.value
}

fn part_2(input: &str) -> i64 {
    // multiply all values by 811589153
    let multiplier = 811589153;

    // mix 10 times
    // part_1() * 10

    // brute force won't work here probably

    let mut values = Vec::new();

    for (i, line) in input.lines().enumerate() {
        let num = line.trim().parse::<i64>().unwrap();

        let val = Node {
            og_order: i,
            value: num * multiplier,
        };

        values.push(val);
    }

    let length = values.len();
    for l in 0..10 {
        println!("{} loop", l);
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
                // let current_value = (current_value as usize) % length;
                for s in 0..current_value {
                    let s = s as usize;
                    let start = (current_index + s) % length;
                    let end = (current_index + s + 1) % length;
                    values.swap(start, end);
                }
            } else {
                // let abs_value = (current_value.abs() as usize) % length;
                let abs_value = current_value.abs();
                for s in 0..abs_value {
                    let s = s as usize;
                    let start = sub_mod(current_index, s, length);
                    let end = sub_mod(current_index, s + 1, length);
                    values.swap(start, end);
                }
            }
        }
    }

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

    // first: 1623178306 second: 3246356612 third: 811589153
    // should be 811589153 2434767459 -1623178306
    println!(
        "first: {} second: {} third: {}",
        first.value, second.value, third.value
    );

    first.value + second.value + third.value
}

fn part_2_smarter(input: &str) -> i64 {
    // https://topaz.github.io/paste/#XQAAAQDOBgAAAAAAAAAFG0sTyga3yMCGEnE4cXAte8gbLEX+YfvmMv85TAjDGfxAWScuAniClINXAsMMCUxnN9KPRregjJe7Y6EIoEi+pnNZbKHQXWtm6NzyxByAEEXDENr7LGl0wWijU8sBgTj66ZwGSrgexQONaZ1rLlwZJYU5P0rGyVn5pyVR5NrKr0SHupMU5t36GvcTFdQNRDFwYPUstaDuCLADIc4rJBmW53NKDFMGHo5prBDIK/m8h9TePMRdHMBl+b2TxnyGOFkIbyYfy0/BOYMM8h6ZHlLNyLH1ZBmq1cFdPGgF0FpYFj0epRnX1cQMv6mHUUhLalZWeTUMfUBBGv5ynYI9VwWOwKDoC30lVZrjjQtLm48CpZjISLt4NKLpMJs4kbDxgrSvcybPcfK8zHkzXAKoDu/nfLRC8/I2Z16bV1uL4O8gRgqQEtzWo2HUstWPwIQyGAtDUHjjbsaitR0CikumHJINapnsxa0I79HVPwugPCPCVXkVBN41WrVolbaU036dUZcFaW58iWvEEKVMNx4MR8o7L2LbW833c1c6kXiEpwX6w1h+JcQvqPfL3F+Cow8bJCYLA2jwtnkWVYxbJaacbYzD3cCPKGPn6fXpytcGp2tIIo1V6OzGo/wBHMYdReJ7QlzQdaVgYxReXzNNyStotDXKFnOxxwHrnFCvxIJuMtj2n56iL78Hqy3seZkrFNjkLoR0GYINgNb7PlRjwMS7jTzObNjI4fSbx1JYOzUUEvsrQwLvHCz7iHHnzEB4+QZgJupA/jXeNrN+rXiIwksGEfbVmRnh/+q6YT0=

    // multiply all values by 811589153
    let multiplier = 811589153;

    // mix 10 times
    // part_1() * 10

    let mut values = Vec::new();
    let mut indicies = Vec::new();

    for (i, line) in input.lines().enumerate() {
        let num = line.trim().parse::<i64>().unwrap();

        values.push(num * multiplier);
        indicies.push(i);
    }

    let length = values.len();
    for _ in 0..10 {
        for i in 0..length {
            let index = indicies[i];
            let value = values[i];
            let new_pos = (index as i64 + value).rem_euclid(length as i64 - 1);
            for ix in indicies.iter_mut() {
                if *ix > index {
                    *ix -= 1;
                }
                if *ix >= new_pos as usize {
                    *ix += 1;
                }
            }
            indicies[i] = new_pos as usize;
        }
    }

    let ordered_zero = indicies[values.iter().position(|&x| x == 0).unwrap()];
    let first_index = (ordered_zero + 1000).rem_euclid(length);
    let second_index = (ordered_zero + 2000).rem_euclid(length);
    let third_index = (ordered_zero + 3000).rem_euclid(length);

    let mut first = 0;
    let mut second = 0;
    let mut third = 0;
    for i in 0..length {
        if indicies[i] == first_index {
            first = values[i];
        }
        if indicies[i] == second_index {
            second = values[i];
        }
        if indicies[i] == third_index {
            third = values[i];
        }
    }

    first + second + third
}

#[derive(Debug, Clone, Copy)]
struct Node {
    og_order: usize,
    value: i64,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]: {} ", self.og_order, self.value)
    }
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
    fn part_2_works() {
        assert_eq!(1623178306, part_2(&BASIC_INPUT_DAY_20));
    }

    #[test]
    fn part_2_smarter_works() {
        assert_eq!(1623178306, part_2_smarter(&BASIC_INPUT_DAY_20));
    }

    #[test]
    fn wrap_helpers() {
        let len = 7;

        assert_eq!(5, add_mod(4, 1, len));
        assert_eq!(0, add_mod(6, 1, len));

        assert_eq!(4, sub_mod(5, 1, len));
        assert_eq!(6, sub_mod(0, 1, len));
    }
}
