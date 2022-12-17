use std::{
    collections::{BTreeSet, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("./inputs/day_17_input.txt").unwrap();

    println!("{}", part_1(&input));
}

fn part_1(input: &str) -> i32 {
    // 7 units wide
    // spawn at x = 2
    // y = highest + 3

    // move left or right
    // fall down
    // move left or right
    // fall down
    // stop if a fall down would hit something.

    // get height after 2022 rocks have settled

    // query a hashset?
    // when a rock stopes, add its points to the hashset
    // to move left, loop over every part of the rock
    // check if there exists something in the hashset that matches
    // if it doesn't, you can move.

    let angle_brackets: Vec<&str> = input.split("").collect();
    let mut movements = Vec::new();
    for item in angle_brackets {
        match item {
            "<" => {
                movements.push(Movement::Left);
            }
            ">" => {
                movements.push(Movement::Right);
            }
            _ => {}
        }
    }

    let mut peak_height = -1;
    let mut settled_rocks = 0;

    let mut rock_index = 0;
    let mut movement_index = 0;

    let mut obstacles = HashSet::new();

    // let mut inner_count = 0;

    while settled_rocks < 2022 {
        // println!("Settled rocks: {}", settled_rocks);
        let mut block = Block::from_rock(get_rock(rock_index), peak_height);
        rock_index += 1;

        loop {
            // println!("block can fall");
            // println!("block: {:?}", block);

            match movements[movement_index] {
                Movement::Left => {
                    if block.can_move_left(&obstacles) {
                        block.move_left();
                        // println!("move left");
                    } else {
                        // hit a wall
                        // don't need to do anything I don't think
                    }
                }
                Movement::Right => {
                    if block.can_move_right(&obstacles) {
                        block.move_right();
                        // println!("move right");
                    } else {
                    }
                }
            }
            // wrap around
            movement_index = (movement_index + 1) % movements.len();

            if block.can_move_down(&obstacles) {
                block.move_down();
                // println!("move down");
            } else {
                // println!("stop rock");
                // add block to obstacles
                settled_rocks += 1;
                let height = block.get_peak_height();
                if height > peak_height {
                    peak_height = height;
                }
                // peak_height = block.get_peak_height();
                // println!("Block stopped. height: {:?}", peak_height);
                for p in block.points.iter() {
                    obstacles.insert(*p);
                }
                break;
            }

            // inner_count += 1;
            // if inner_count > 100 {
            // return 99;

            // }
        }
    }

    // println!("obstacles: {:?}", &obstacles);
    // draw_obstacles(&obstacles, peak_height);

    peak_height + 1
}

fn part_2(input: &str) -> i32 {
    99
}

fn draw_obstacles(obstacles: &HashSet<Point>, height: i32) {
    for y in (0..=(height + 5)).rev() {
        let mut line = "|".to_string();
        for x in 0..7 {
            let p = Point::new(x, y);
            if obstacles.contains(&p) {
                line = format!("{}{}", line, "#");
            } else {
                line = format!("{}{}", line, ".");
            }
        }
        println!("{}|", line);
    }
}

fn get_rock(index: u32) -> Rock {
    match index % 5 {
        0 => Rock::Flat,
        1 => Rock::Plus,
        2 => Rock::L,
        3 => Rock::Line,
        4 => Rock::Square,
        _ => panic!(),
    }
}

#[derive(Debug)]
struct Block {
    rock: Rock,
    points: Vec<Point>,
}

impl Block {
    fn from_rock(rock: Rock, peak_height: i32) -> Self {
        match rock {
            Rock::Flat => Block {
                rock,
                points: vec![
                    Point::new(2, peak_height + 4),
                    Point::new(3, peak_height + 4),
                    Point::new(4, peak_height + 4),
                    Point::new(5, peak_height + 4),
                ],
            },
            Rock::Plus => Block {
                rock,
                points: vec![
                    Point::new(3, peak_height + 6),
                    Point::new(2, peak_height + 5),
                    Point::new(3, peak_height + 5),
                    Point::new(4, peak_height + 5),
                    Point::new(3, peak_height + 4),
                ],
            },
            Rock::L => Block {
                rock,
                points: vec![
                    Point::new(4, peak_height + 6),
                    Point::new(4, peak_height + 5),
                    Point::new(4, peak_height + 4),
                    Point::new(3, peak_height + 4),
                    Point::new(2, peak_height + 4),
                ],
            },
            Rock::Line => Block {
                rock,
                points: vec![
                    Point::new(2, peak_height + 7),
                    Point::new(2, peak_height + 6),
                    Point::new(2, peak_height + 5),
                    Point::new(2, peak_height + 4),
                ],
            },
            Rock::Square => Block {
                rock,
                points: vec![
                    Point::new(2, peak_height + 5),
                    Point::new(2, peak_height + 4),
                    Point::new(3, peak_height + 5),
                    Point::new(3, peak_height + 4),
                ],
            },
        }
    }

    fn can_move_left(&self, obstacles: &HashSet<Point>) -> bool {
        for p in &self.points {
            let left_p = p.left();
            if !left_p.is_in_bounds() {
                return false;
            }
            if obstacles.contains(&left_p) {
                return false;
            }
        }

        return true;
    }

    fn can_move_right(&self, obstacles: &HashSet<Point>) -> bool {
        for p in &self.points {
            let right_p = p.right();
            if !right_p.is_in_bounds() {
                return false;
            }
            if obstacles.contains(&right_p) {
                return false;
            }
        }

        return true;
    }

    fn can_move_down(&self, obstacles: &HashSet<Point>) -> bool {
        for p in &self.points {
            let down_p = p.down();
            if !down_p.is_in_bounds() {
                return false;
            }
            if obstacles.contains(&down_p) {
                return false;
            }
        }

        return true;
    }

    fn move_left(&mut self) {
        for p in self.points.iter_mut() {
            p.move_left();
        }
    }

    fn move_right(&mut self) {
        for p in self.points.iter_mut() {
            p.move_right();
        }
    }

    fn move_down(&mut self) {
        for p in self.points.iter_mut() {
            p.move_down();
        }
    }

    fn get_peak_height(&self) -> i32 {
        // first element is always the highest
        // that's the way they are constructed
        self.points[0].y
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn left(&self) -> Self {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn right(&self) -> Self {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn down(&self) -> Self {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn move_left(&mut self) {
        self.x -= 1;
    }

    fn move_right(&mut self) {
        self.x += 1;
    }

    fn move_down(&mut self) {
        self.y -= 1;
    }

    fn is_in_bounds(&self) -> bool {
        if self.x < 0 || self.x > 6 || self.y < 0 {
            false
        } else {
            true
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Rock {
    Flat,
    Plus,
    L,
    Line,
    Square,
}

#[derive(Debug)]
enum Movement {
    Left,
    Right,
}

#[cfg(test)]
mod tests {

    use super::*;

    const BASIC_INPUT_DAY_17: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn part_1_works() {
        assert_eq!(3068, part_1(&BASIC_INPUT_DAY_17));
    }

    #[test]
    fn part_2_works() {
        // 1514285714288
        assert_eq!(0, part_2(&BASIC_INPUT_DAY_17));
    }

    #[test]
    fn test_rock_order() {
        let mut rock = get_rock(0);
        assert_eq!(Rock::Flat, rock);

        rock = get_rock(1);
        assert_eq!(Rock::Plus, rock);

        rock = get_rock(2);
        assert_eq!(Rock::L, rock);

        rock = get_rock(3);
        assert_eq!(Rock::Line, rock);

        rock = get_rock(4);
        assert_eq!(Rock::Square, rock);

        rock = get_rock(5);
        assert_eq!(Rock::Flat, rock);
    }

    #[test]
    fn spawn_block_can_move() {
        let block = Block::from_rock(Rock::Flat, -1);

        let obstacles = HashSet::new();

        assert!(block.can_move_down(&obstacles));
        assert!(block.can_move_left(&obstacles));
        assert!(block.can_move_right(&obstacles));
    }

    #[test]
    fn block_movement_left_stops() {
        let mut block = Block::from_rock(Rock::Flat, -1);

        let obstacles = HashSet::new();

        block.move_left();
        block.move_left();
        // hit a wall
        assert_eq!(false, block.can_move_left(&obstacles));
    }

    #[test]
    fn block_movement_right_stops() {
        let mut block = Block::from_rock(Rock::Flat, -1);

        let obstacles = HashSet::new();

        block.move_right();
        // block.move_right();
        // hit a wall
        assert_eq!(false, block.can_move_right(&obstacles));
    }

    #[test]
    fn block_movement_down_stops() {
        let mut block = Block::from_rock(Rock::Flat, -1);

        let obstacles = HashSet::new();

        block.move_down();
        block.move_down();
        block.move_down();
        // hit the floor
        assert_eq!(false, block.can_move_down(&obstacles));
    }
}
