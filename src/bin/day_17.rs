use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("./inputs/day_17_input.txt").unwrap();

    // println!("{}", part_1(&input)); // 3232
    println!("{}", part_2(&input));
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
    // 40 movements
    // println!("movement length: {:?}", movements.len());

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
    draw_obstacles(&obstacles, peak_height);

    peak_height
}

fn part_2(input: &str) -> i32 {
    // does it repeat after 200 iterations?
    // 40 inputs, 5 blocks
    // simulate 200
    // get the height.
    // figure out the overlap

    // 1T / 200 = x
    // height * x is the answer?

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
    // 40 movements
    // 10091 for real input
    println!("movement length: {:?}", movements.len());

    let mut peak_height = -1;
    let mut settled_rocks = 0;

    let mut rock_index = 0;
    let mut movement_index = 0;

    let mut obstacles = HashSet::new();

    // let mut inner_count = 0;

    while settled_rocks < 1180 {
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
            // if movement_index == 0 && rock_index % 5 == 0 {
            //     println!("movement at 0. count: {} rock_index mod: {} height: {}", settled_rocks, rock_index, peak_height);
            // }
            // when these match, height goes up by ~27
            // 22 + 28 + 27 + 26 + 27 + 26 + 27 + 26 + 27 + 26 + 27
            // movement at 0. count: 14 rock_index mod: 15 height: 22
            // +15 +15 +28
            // movement at 0. count: 29 rock_index mod: 30 height: 50
            // +20 +20 +27
            // movement at 0. count: 49 rock_index mod: 50 height: 77
            // +15 +15 +26
            // movement at 0. count: 64 rock_index mod: 65 height: 103
            // +20 +20 +27
            // movement at 0. count: 84 rock_index mod: 85 height: 130
            // +15 +15 +26
            // movement at 0. count: 99 rock_index mod: 100 height: 156
            // +20 +20 +27
            // movement at 0. count: 119 rock_index mod: 120 height: 183
            // +15 +15 +26
            // movement at 0. count: 134 rock_index mod: 135 height: 209
            // +20 +20 +27
            // movement at 0. count: 154 rock_index mod: 155 height: 236
            // +15 +15 +26
            // movement at 0. count: 169 rock_index mod: 170 height: 262
            // +20 +20 +27
            // movement at 0. count: 189 rock_index mod: 190 height: 289
            // every 15 settled rocks, +26 height
            // every 20 settled rocks, +27 height
            // every 35 rocks, + 53 height
            // after the first 29 rocks and 50 height

            // 2022 - 35 = 1987
            // 1987 / 45 = 44.1555
            // 45 * 53 = 2385
            // 50 + 2288 = 2338
            // 50 + 2385 = 2435
            // real 3068
            // off by 633

            // 2022 - 29 = 1993
            // 1993 / 35 = 56.94
            // 56 * 53 = 2968
            // off by 100
            // 35 * 56 = 1960
            // 1960 + 29 = 1989
            // 33 to evaluate at the end
            // can account for +15 -> +26
            // or +20 -> 27
            // 50 + 2968 + 26 + 27 = 3071
            // off by 3

            // pattern keeps going to 2024 as expected
            // why is it +15 +20?
            // 15 isn't a factor of 40 or 200
            // 15 * 20 = 300

            // Part 2
            // Pattern
            // no movement == 0 && rock == 0
            // in 2m iterations

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

                let new_line = line_logic(&mut obstacles, height);
                if let Some(new_line) = new_line {
                    if new_line == 275 {
                        println!("line 275 filled at rocks: {}", settled_rocks);
                        draw_obstacles_window(&obstacles, new_line);
                    } else if new_line == 292 {
                        println!("line 292 filled at rocks: {}", settled_rocks);
                        draw_obstacles_window(&obstacles, new_line);
                    } else if new_line == 3034 {
                        println!("line 3034 filled at rocks: {}", settled_rocks);
                        draw_obstacles_window(&obstacles, new_line);
                    } else if new_line == 5793 {
                        println!("line 5793 filled at rocks: {}", settled_rocks);
                        draw_obstacles_window(&obstacles, new_line);
                    }
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

    // 307

    // bottom
    // |#####..|
    // |..###..|
    // |...#...|
    // |..####.|
    // top
    // |.......|
    // |.##....|
    // |.##...#|
    // |..#...#|
    // |..#.###|
    // |..#..#.|
    // |..#.###|
    // |.#####.|
    // |....#..|

    // should stack nicely
    // 2022 / 200 = 10.11
    // 307 * 10.11 = 3103.77
    // test == 3068
    // diff of 35

    // println!("Height after 200: {}", peak_height);

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

    // 1585632184069 too high
    // -1 still too high
    // 1585632183914 is too low
    // 1585632183915 is correct
    println!(
        "Height after 1T: {}",
        naive_height + (peak_height + 1) as u64
    );

    peak_height
}

fn part_2_broken(input: &str) -> i32 {
    // probably need to figure out when this repeats
    // there are 40 movements
    // 5 blocks
    // each block will use x movements

    // maybe whenever I get floor, I can set that to the new floor and restart?

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
    // 40 movements
    // println!("movement length: {:?}", movements.len());

    let mut peak_height = -1;
    let mut settled_rocks = 0;

    let mut rock_index = 0;
    let mut movement_index = 0;

    let mut obstacles = HashSet::new();

    // let mut inner_count = 0;

    let count: u64 = 1_000_000_000_000;
    let height: u64 = 1514285714288;

    let mut heights = Vec::new();

    // takes ~25s
    let million = 1_000_000;
    // 1B would take 7 hours
    // 1T would take 7000 hours = 289 days

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

                // was a line made??
                // if yes, where?
                let new_line = line_logic(&mut obstacles, height);
                if let Some(new_line) = new_line {
                    // let line_diff = peak_height - new_line;

                    // draw old
                    // draw_obstacles(&obstacles, peak_height);

                    heights.push(peak_height);

                    cull_obstacles(&mut obstacles, new_line, peak_height);

                    // draw new
                    // println!("New obstacles");
                    // peak_height = 0;
                    peak_height -= new_line;
                    // println!("new peak: {}", peak_height);
                    // draw_obstacles(&obstacles, peak_height);
                    // if heights.len() == 2 {
                    //     println!("heights: {:?}", heights);
                    //     return 99;
                    // }
                    // return 99;
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
    draw_obstacles(&obstacles, peak_height);

    // len 59
    // len 19
    // len 21
    println!("heights: {:?} len: {}", heights, heights.len());
    let mut sum: u64 = 0;
    for h in heights {
        sum += h as u64;
    }

    // should be 3068 for 2022 blocks
    // 3413
    // off by 345
    // 3024
    // off by 44
    // 3083
    // off by 15

    // I was running, not testing
    // test input repeats, but doesn't make lines

    // should be 3232 for real input
    // 3083 + 118, len 21
    // off by 149
    // adding peak and len is 3222. Off by 10
    // does that make sense?

    // 3122 + 98. len 35
    // 3190 + 179. len 23
    // 3034 + 156. len 28

    println!("Calculated height: {}", sum);
    peak_height
}

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
    #[ignore = "done for now"]
    fn part_1_works() {
        assert_eq!(3068, part_1(&BASIC_INPUT_DAY_17));
    }

    #[test]
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
