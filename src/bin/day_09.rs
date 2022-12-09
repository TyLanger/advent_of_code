use std::{collections::HashSet, fs};

fn main() {
    // remember to change the input
    let file = fs::read_to_string("./inputs/day_09_input.txt").unwrap();
    // 5908 is too high
    println!("{}", part_1(&file));
    // println!("{}", part_2(&file));
}

fn part_1(input: &str) -> usize {
    // each line moves the head in a direction x tiles

    // need to find unique positions visited by the tail
    // seems like a set
    let mut tail_positions: HashSet<Position> = HashSet::new();

    let mut head_pos = Position::new(0, 0);
    let mut tail_pos = Position::new(0, 0);

    // bug here
    // was just testing insertion
    // forgot to remove it
    // in the test input, it visits 3,4 so it's fine
    // in the real input, it doesn't
    // lol weird bug
    // tail_positions.insert(Position::new(3, 4));

    let lines = input.lines();
    for line in lines {
        let (letter, number) = line.split_once(" ").unwrap();
        let amount = number.parse().unwrap();
        match letter {
            "U" => {
                for _ in 0..amount {
                    head_pos.y += 1;
                    tail_pos.calculate_follow_pos(head_pos);
                    tail_positions.insert(tail_pos);
                }
            }
            "R" => {
                for _ in 0..amount {
                    head_pos.x += 1;
                    tail_pos.calculate_follow_pos(head_pos);
                    tail_positions.insert(tail_pos);
                }
            }
            "D" => {
                for _ in 0..amount {
                    head_pos.y -= 1;
                    tail_pos.calculate_follow_pos(head_pos);
                    tail_positions.insert(tail_pos);
                }
            }
            "L" => {
                for _ in 0..amount {
                    head_pos.x -= 1;
                    tail_pos.calculate_follow_pos(head_pos);
                    tail_positions.insert(tail_pos);
                }
            }
            _ => panic!("bad input"),
        }
    }

    tail_positions.len()
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    fn calculate_follow_pos(&mut self, head: Position) {
        let x_diff = head.x - self.x;
        let y_diff = head.y - self.y;

        // if head is within the nearby 3x3 grid, do nothing
        if x_diff.abs() < 2 && y_diff.abs() < 2 {
            // do nothing
        } else if x_diff.abs() > 0 && y_diff.abs() > 0 {
            // diagonal
            let x_change = if x_diff > 0 { 1 } else { -1 };
            let y_change = if y_diff > 0 { 1 } else { -1 };
            self.x += x_change;
            self.y += y_change;
        } else {
            // 3 cases: +1, 0, -1
            if x_diff > 1 {
                self.x += 1;
            } else if x_diff < -1 {
                self.x -= 1;
            }
            if y_diff > 1 {
                self.y += 1;
            } else if y_diff < -1 {
                self.y -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const DAY_9_BASIC_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    // #[ignore = "not ready"]
    fn part_1_works() {
        let result = part_1(&DAY_9_BASIC_INPUT);

        assert_eq!(13, result);
    }

    #[test]
    fn tail_doesnt_move() {
        let head = Position::new(0, 0);
        let mut tail = Position::new(0, 0);
        tail.calculate_follow_pos(head);

        assert_eq!(Position::new(0, 0), tail);

        let head = Position::new(0, 1);
        let mut tail = Position::new(0, 0);
        tail.calculate_follow_pos(head);

        assert_eq!(Position::new(0, 0), tail);

        let head = Position::new(1, 1);
        let mut tail = Position::new(0, 0);
        tail.calculate_follow_pos(head);

        assert_eq!(Position::new(0, 0), tail);

        let head = Position::new(1, 0);
        let mut tail = Position::new(0, 0);
        tail.calculate_follow_pos(head);

        assert_eq!(Position::new(0, 0), tail);

        let head = Position::new(1, -1);
        let mut tail = Position::new(0, 0);
        tail.calculate_follow_pos(head);

        assert_eq!(Position::new(0, 0), tail);

        let head = Position::new(0, -1);
        let mut tail = Position::new(0, 0);
        tail.calculate_follow_pos(head);

        assert_eq!(Position::new(0, 0), tail);

        let head = Position::new(-1, -1);
        let mut tail = Position::new(0, 0);
        tail.calculate_follow_pos(head);

        assert_eq!(Position::new(0, 0), tail);

        let head = Position::new(-1, 0);
        let mut tail = Position::new(0, 0);
        tail.calculate_follow_pos(head);

        assert_eq!(Position::new(0, 0), tail);

        let head = Position::new(-1, 1);
        let mut tail = Position::new(0, 0);
        tail.calculate_follow_pos(head);

        assert_eq!(Position::new(0, 0), tail);
    }

    #[test]
    fn tail_moves_cardinal() {
        let head = Position::new(2, 0);
        let mut tail = Position::new(0, 0);
        tail.calculate_follow_pos(head);

        assert_eq!(Position::new(1, 0), tail);

        let head = Position::new(-2, 0);
        let mut tail = Position::new(0, 0);
        tail.calculate_follow_pos(head);

        assert_eq!(Position::new(-1, 0), tail);

        let head = Position::new(0, 2);
        let mut tail = Position::new(0, 0);
        tail.calculate_follow_pos(head);

        assert_eq!(Position::new(0, 1), tail);

        let head = Position::new(0, -2);
        let mut tail = Position::new(0, 0);
        tail.calculate_follow_pos(head);

        assert_eq!(Position::new(0, -1), tail);
    }

    #[test]
    fn tail_moves_diagonal() {
        let head = Position::new(2, 1);
        let mut tail = Position::new(0, 0);
        tail.calculate_follow_pos(head);

        assert_eq!(Position::new(1, 1), tail);

        let head = Position::new(2, -1);
        let mut tail = Position::new(0, 0);
        tail.calculate_follow_pos(head);

        assert_eq!(Position::new(1, -1), tail);

        let head = Position::new(-2, 1);
        let mut tail = Position::new(0, 0);
        tail.calculate_follow_pos(head);

        assert_eq!(Position::new(-1, 1), tail);

        let head = Position::new(-2, -1);
        let mut tail = Position::new(0, 0);
        tail.calculate_follow_pos(head);

        assert_eq!(Position::new(-1, -1), tail);

        let head = Position::new(1, 2);
        let mut tail = Position::new(0, 0);
        tail.calculate_follow_pos(head);

        assert_eq!(Position::new(1, 1), tail);

        let head = Position::new(1, -2);
        let mut tail = Position::new(0, 0);
        tail.calculate_follow_pos(head);

        assert_eq!(Position::new(1, -1), tail);

        let head = Position::new(-1, 2);
        let mut tail = Position::new(0, 0);
        tail.calculate_follow_pos(head);

        assert_eq!(Position::new(-1, 1), tail);

        let head = Position::new(-1, -2);
        let mut tail = Position::new(0, 0);
        tail.calculate_follow_pos(head);

        assert_eq!(Position::new(-1, -1), tail);
    }

    #[test]
    fn tail_starting_non_zero() {
        let head = Position::new(-11, -2);
        let mut tail = Position::new(-10, 0);
        tail.calculate_follow_pos(head);

        assert_eq!(Position::new(-11, -1), tail);
    }
}
