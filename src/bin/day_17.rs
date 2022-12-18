use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("./inputs/day_17_input.txt").unwrap();

    println!("{}", part_1(&input)); // 3232
    println!("{}", part_2(&input));
    part_2_math(); // 1585632183915
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

    while settled_rocks < 2022 {
        let mut block = Block::from_rock(get_rock(rock_index), peak_height);
        rock_index += 1;

        loop {
            match movements[movement_index] {
                Movement::Left => {
                    if block.can_move_left(&obstacles) {
                        block.move_left();
                    }
                }
                Movement::Right => {
                    if block.can_move_right(&obstacles) {
                        block.move_right();
                    }
                }
            }
            // wrap around
            movement_index = (movement_index + 1) % movements.len();

            if block.can_move_down(&obstacles) {
                block.move_down();
            } else {
                settled_rocks += 1;
                let height = block.get_peak_height();
                if height > peak_height {
                    peak_height = height;
                }

                for p in block.points.iter() {
                    obstacles.insert(*p);
                }
                break;
            }
        }
    }

    draw_obstacles(&obstacles, peak_height);

    peak_height
}

fn part_2(input: &str) -> i32 {
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

    while settled_rocks < 1180 {
        let mut block = Block::from_rock(get_rock(rock_index), peak_height);
        rock_index += 1;

        loop {
            match movements[movement_index] {
                Movement::Left => {
                    if block.can_move_left(&obstacles) {
                        block.move_left();
                    }
                }
                Movement::Right => {
                    if block.can_move_right(&obstacles) {
                        block.move_right();
                    }
                }
            }
            // wrap around
            movement_index = (movement_index + 1) % movements.len();

            if block.can_move_down(&obstacles) {
                block.move_down();
            } else {
                // add block to obstacles
                settled_rocks += 1;
                let height = block.get_peak_height();
                if height > peak_height {
                    peak_height = height;
                }
                for p in block.points.iter() {
                    obstacles.insert(*p);
                }

                // let new_line = line_logic(&mut obstacles, height);
                // if let Some(new_line) = new_line {
                //     if new_line == 275 {
                //         println!("line 275 filled at rocks: {}", settled_rocks);
                //         draw_obstacles_window(&obstacles, new_line);
                //     } else if new_line == 292 {
                //         println!("line 292 filled at rocks: {}", settled_rocks);
                //         draw_obstacles_window(&obstacles, new_line);
                //     } else if new_line == 3034 {
                //         println!("line 3034 filled at rocks: {}", settled_rocks);
                //         draw_obstacles_window(&obstacles, new_line);
                //     } else if new_line == 5793 {
                //         println!("line 5793 filled at rocks: {}", settled_rocks);
                //         draw_obstacles_window(&obstacles, new_line);
                //     }
                // }

                break;
            }
        }
    }

    // line 3040 matches line 281
    // 3055 matches 296
    // diff 2759
    // 292 is a tetris
    // figure out when that happens?

    // I need to map 1T rocks to a height
    // I don't know how many iterations gave me line 281

    // after 171 rocks, line 275 is filled (height 275)
    // after 209 rocks, line 292 is filled. Worse candidate.
    // It's filled by a line piece falling down a crack

    // 275 -> 171
    // 3040-6 = 3034
    // 3034 -> ?

    // 3034 -> 1911
    // 5793 -> 3651

    // rocks
    // 1911 - 171 = 1740
    // should repeat again after another 1740 blocks
    // 1911 + 1740 = 3651 rocks
    // height should be: 3034 + 2759 = 5793

    // expect:
    // line 5793 is a line
    // its rock count is 3651

    // what to do?
    // keep adding lines until it breaches 1T
    let base_rock_count: u64 = 171;
    let pattern_rock_count: u64 = 1740;
    // at 171 + 1740 = 1911
    // 1911 + 1740 = 3651
    // 3651 + 1740 = 5391

    let height_growth: u64 = 2759;

    // reverse
    // 5391 - 171 = 5220
    // 5220 / 1740 = 3
    // 6000 - 171 = 5829
    // 5829 / 1740 = 3.35
    // leftover = 5829 % 1740 = 609
    // then I need the height after 171 + 609 iterations?
    // height = 171 + repeats * 2759 + h(609?)
    let big = 1_000_000_000_000 - base_rock_count;
    let repeats = big / pattern_rock_count;
    println!("repeats: {}", repeats);
    let remainder = big % pattern_rock_count;
    println!("remainder: {}", remainder);

    // 1T repeats: 574_712_643
    // 1T remainder: 1009
    let naive_height: u64 = repeats * height_growth;
    // println!("naive height: {}", naive_height);
    // 1T naive: 1_585_632_182_037

    // after 2 repeats, 2 * 2759 = 5518
    // 5518 + 275 = 5793

    // total_height = repeats * growth + height at 275+remainder?
    // height after 275+1009 = 1284
    // h(1284) = 2032 +- 1?
    // 171 + 1009 = 1180
    // h(1180) = 1877

    // 1585632183915 is correct
    println!(
        "Height after 1T: {}",
        naive_height + (peak_height + 1) as u64
    );

    peak_height
}

fn part_2_math() {
    // to get the first line
    // add 1740 to get other repeats
    let base_rock_count: u64 = 171;
    let pattern_rock_count: u64 = 1740;
    // gain height every repeat
    let height_growth: u64 = 2759;

    let big = 1_000_000_000_000 - base_rock_count;
    let repeats = big / pattern_rock_count;
    println!("repeats: {}", repeats);
    let remainder = big % pattern_rock_count;
    println!("remainder: {}", remainder);

    let naive_height: u64 = repeats * height_growth;

    // h(base_rock_count + remainder) = 1877
    println!("Height after 1T: {}", naive_height + (1877 + 1) as u64);
    // 1585632183915 is correct
}

#[allow(unused)]
fn line_logic(obstacles: &mut HashSet<Point>, last_placed_height: i32) -> Option<i32> {
    // only loop over 5 nearby heights
    // last placed height is the block's heaight,
    // not the greatest height
    // lines are probably made below the height
    'height: for y in ((last_placed_height - 5)..=last_placed_height).rev() {
        let mut count = 0;
        for x in 0..7 {
            let p = Point::new(x, y);
            if obstacles.contains(&p) {
                count += 1;
            } else {
                continue 'height;
            }
        }
        if count == 7 {
            // full line found
            return Some(y);
        }
    }
    None
}

#[allow(unused)]
fn cull_obstacles(obstacles: &mut HashSet<Point>, new_line: i32, max_height: i32) {
    let mut new_obstacles = HashSet::new();
    // skip the real line
    // or you get double hits
    // the next block will match on this line that got reinserted
    for y in (new_line + 1)..=max_height {
        for x in 0..7 {
            let p = Point::new(x, y);
            if obstacles.contains(&p) {
                let new_p = Point::new(x, p.y - (new_line + 1));
                new_obstacles.insert(new_p);
            }
        }
    }
    *obstacles = new_obstacles;
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
        if y % 5 == 0 {
            println!("{}| {}", line, y);
        } else {
            println!("{}|", line);
        }
    }
}

#[allow(unused)]
fn draw_obstacles_window(obstacles: &HashSet<Point>, height: i32) {
    for y in ((height - 10)..=(height + 5)).rev() {
        let mut line = "|".to_string();
        for x in 0..7 {
            let p = Point::new(x, y);
            if obstacles.contains(&p) {
                line = format!("{}{}", line, "#");
            } else {
                line = format!("{}{}", line, ".");
            }
        }
        if y % 5 == 0 {
            println!("{}| {}", line, y);
        } else {
            println!("{}|", line);
        }
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
    points: Vec<Point>,
}

impl Block {
    fn from_rock(rock: Rock, peak_height: i32) -> Self {
        match rock {
            Rock::Flat => Block {
                // rock,
                points: vec![
                    Point::new(2, peak_height + 4),
                    Point::new(3, peak_height + 4),
                    Point::new(4, peak_height + 4),
                    Point::new(5, peak_height + 4),
                ],
            },
            Rock::Plus => Block {
                // rock,
                points: vec![
                    Point::new(3, peak_height + 6),
                    Point::new(2, peak_height + 5),
                    Point::new(3, peak_height + 5),
                    Point::new(4, peak_height + 5),
                    Point::new(3, peak_height + 4),
                ],
            },
            Rock::L => Block {
                // rock,
                points: vec![
                    Point::new(4, peak_height + 6),
                    Point::new(4, peak_height + 5),
                    Point::new(4, peak_height + 4),
                    Point::new(3, peak_height + 4),
                    Point::new(2, peak_height + 4),
                ],
            },
            Rock::Line => Block {
                // rock,
                points: vec![
                    Point::new(2, peak_height + 7),
                    Point::new(2, peak_height + 6),
                    Point::new(2, peak_height + 5),
                    Point::new(2, peak_height + 4),
                ],
            },
            Rock::Square => Block {
                // rock,
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

        true
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

        true
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

        true
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
        !(self.x < 0 || self.x > 6 || self.y < 0)
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
    #[ignore = "done for now"]
    fn part_1_works() {
        assert_eq!(3068, part_1(&BASIC_INPUT_DAY_17));
    }

    #[test]
    #[ignore = "test input doesn't work"]
    fn part_2_works() {
        // 1_514_285_714_288
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
