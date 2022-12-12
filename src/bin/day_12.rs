use std::{fs, collections::BinaryHeap};

fn main() {
    let input = fs::read_to_string("./inputs/day_12_input.txt").unwrap();

    println!("{}", part_1(&input));
}

fn part_1(input: &str) -> u32 {
    // a is the lowest
    // z is the highest

    // do I have to implement a*?
    // would flow field work?
    // let characters: Vec<char> = input.chars().collect();

    let width = input.lines().next().unwrap().trim().len();
    dbg!(width);

    let mut nodes: Vec<Node> = input
        .chars()
        .filter(|x| !x.is_whitespace())
        .map(|c| {
            let value;
            let sequence;
            if c == 'S' {
                value = 1;
                sequence = Sequence::Start;
            } else if c == 'E' {
                value = 26;
                sequence = Sequence::End;
            } else {
                value = letter_to_value(c);
                sequence = Sequence::Normal;
            }

            Node {
                value,
                neighbour_indicies: Vec::new(),
                sequence,
            }
        })
        .collect();

    let mut start_index = 0;
    let mut end_index = 0;

    for i in 0..nodes.len() {
        match nodes[i].sequence {
            Sequence::Start => start_index = i,
            Sequence::End => end_index = i,
            Sequence::Normal => {}
        }
        if let Some(up) = get_up_index(i, width) {
            // is the neightbour at most 1 bigger?
            if valid_neighbour(nodes[i].value, nodes[up].value) {
                nodes[i].neighbour_indicies.push(up);
            }
        }
        if let Some(right) = get_right_index(i, width, nodes.len()) {
            if valid_neighbour(nodes[i].value, nodes[right].value) {
                nodes[i].neighbour_indicies.push(right);
            }
        }
        if let Some(down) = get_down_index(i, width, nodes.len()) {
            if valid_neighbour(nodes[i].value, nodes[down].value) {
                nodes[i].neighbour_indicies.push(down);
            }
        }
        if let Some(left) = get_left_index(i, width) {
            if valid_neighbour(nodes[i].value, nodes[left].value) {
                nodes[i].neighbour_indicies.push(left);
            }
        }
    }

    dbg!(start_index, end_index);

    // a* with the heuristic being your value?
    // from a=1, it will take at least 25 moves
    for i in 0..nodes.len() {
        println!("{:?}", nodes[i].neighbour_indicies);
    }

    // start at start_index
    // are you at the end?
    // no?
    // add neighbours to the open set
    // add self to closed set
    // find smallest neighbour
    // that's the new next
    // next is the lowest f_score
    // f_score is g + h
    // g is current cost

    // let open_set = BinaryHeap::new();

    99
}

fn letter_to_value(letter: char) -> u32 {
    letter as u32 - 96
}

fn get_up_index(i: usize, width: usize) -> Option<usize> {
    if i > width {
        let up = i - width;
        Some(up)
    } else {
        None
    }
}

fn get_right_index(i: usize, width: usize, max_len: usize) -> Option<usize> {
    if i < max_len && i % width != (width - 1) {
        let right = i + 1;
        Some(right)
    } else {
        None
    }
}

fn get_down_index(i: usize, width: usize, max_len: usize) -> Option<usize> {
    if i + width < max_len {
        let down = i + width;
        Some(down)
    } else {
        None
    }
}

fn get_left_index(i: usize, width: usize) -> Option<usize> {
    if i > 0 && i % width != 0 {
        let left = i - 1;
        Some(left)
    } else {
        None
    }
}

fn valid_neighbour(my_value: u32, their_value: u32) -> bool {
    if their_value <= my_value {
        true
    } else if their_value - my_value == 1 {
        true
    } else {
        false
    }
}

struct Node {
    value: u32,
    neighbour_indicies: Vec<usize>,
    sequence: Sequence,
}

enum Sequence {
    Start,
    End,
    Normal,
}

#[cfg(test)]
mod tests {

    use super::*;

    const BASIC_INPUT_DAY_12: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn part_1_works() {
        assert_eq!(31, part_1(&BASIC_INPUT_DAY_12));
    }

    #[test]
    fn letter_to_value_test() {
        assert_eq!(1, letter_to_value('a'));
        assert_eq!(26, letter_to_value('z'));
    }

    #[test]
    fn get_neighbour_indicies() {
        // 0,1,2
        // 3,4,5
        // 6,7,8

        // top left
        assert_eq!(None, get_up_index(0, 3));
        assert_eq!(Some(1), get_right_index(0, 3, 9));
        assert_eq!(Some(3), get_down_index(0, 3, 9));
        assert_eq!(None, get_left_index(0, 3));

        // top right
        assert_eq!(None, get_up_index(2, 3));
        assert_eq!(None, get_right_index(2, 3, 9));
        assert_eq!(Some(5), get_down_index(2, 3, 9));
        assert_eq!(Some(1), get_left_index(2, 3));

        // center
        assert_eq!(Some(1), get_up_index(4, 3));
        assert_eq!(Some(5), get_right_index(4, 3, 9));
        assert_eq!(Some(7), get_down_index(4, 3, 9));
        assert_eq!(Some(3), get_left_index(4, 3));

        // bottom right
        assert_eq!(Some(3), get_up_index(6, 3));
        assert_eq!(Some(7), get_right_index(6, 3, 9));
        assert_eq!(None, get_down_index(6, 3, 9));
        assert_eq!(None, get_left_index(6, 3));

        // bottom left
        assert_eq!(Some(5), get_up_index(8, 3));
        assert_eq!(None, get_right_index(8, 3, 9));
        assert_eq!(None, get_down_index(8, 3, 9));
        assert_eq!(Some(7), get_left_index(8, 3));
    }

    #[test]
    fn valid_neighbour_test() {
        let me = 1;
        let them = 2;

        assert_eq!(true, valid_neighbour(me, them));
        assert_eq!(true, valid_neighbour(26, 26));
        assert_eq!(true, valid_neighbour(26, 1));
        assert_eq!(false, valid_neighbour(5, 20));
        assert_eq!(false, valid_neighbour(5, 7));
    }
}
