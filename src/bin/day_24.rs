use std::{fs, collections::{HashSet, HashMap}};

fn main() {
    let input = fs::read_to_string("./inputs/day_24_input.txt").unwrap();

    println!("{}", part_1(&input));
}

fn part_1(input: &str) -> u32 {
    // could do 3D pathfinding
    // each time step is a new slice of the 3d grid
    // can only go forward in time.

    // simulate next minute
    // check what spaces you can move to
    // visit each of them and simulate the next minute
    // some will end in dead ends with no available moves

    // each row will reset after x iterations
    // each col will repeat after y iterations
    // can figure out the blizzards around a point by
    // moving each blizzard in the nearby rows and columns that many spaces
    // at minute 5, the blizzard will have moved 5 spaces

    // at each minute,
    // calc the possible spots you can be
    // at t=1, you can be
    // start, down 1
    // at t=2, you can be
    // start, down 1, down 2
    // save theses possible positions
    // iterate over all of them
    // get the new positions you can be in
    // when the final pos exists, you win
    // return t

    // parse
    let mut width = 0;
    let mut height = 0;

    let mut blizzards = Vec::new();

    for (i, line) in input.lines().skip(1).enumerate() {
        let split: Vec<&str> = line.split("").skip(1).collect();
        // println!("split: {:?}", split);

        // skip the wall
        for (j, &s) in split.iter().skip(1).enumerate() {
            let mut direction = Direction::Up;
            match s {
                "#" => {
                    continue;
                }
                "." => {
                    continue;
                }
                ">" => {
                    direction = Direction::Right;
                }
                "<" => {
                    direction = Direction::Left;
                }
                "v" => {
                    direction = Direction::Down;
                }
                "^" => {
                    direction = Direction::Up;
                }
                _ => {
                    continue;
                }
            }

            let position = Position {
                x: j as i32,
                y: i as i32,
            };

            let blizz = Blizzard {
                direction,
                position,
            };
            blizzards.push(blizz);
        }

        width = split.len() - 1;
        height = i;
    }
    width -= 2;
    // exclude the # walls
    println!("Size: w: {}, h: {}", width, height);

    // println!("Blizzards: {:?}", blizzards);
    draw_blizzards(&blizzards, width, height);

    for m in 1..=18 {
        println!("Minute {}", m);
        simulate(&mut blizzards, width as u32, height as u32);
        draw_blizzards(&blizzards, width, height);
    }
    


    99
}

fn simulate(blizzards: &mut Vec<Blizzard>, width: u32, height: u32) {
    for b in blizzards {
        b.sim(width, height);
    }
}

fn get_blizzard_map(blizzards: &Vec<Blizzard>) -> HashMap<Position, BlizzardDisplay> {
    let mut map: HashMap<Position, BlizzardDisplay> = HashMap::new();

    for &b in blizzards {
        let get = map.get(&b.position);
        if let Some(display) = get {
            let count = match display {
                BlizzardDisplay::Direction(_) => 2,
                BlizzardDisplay::Count(n) => n+1,
            };
            map.insert(b.position, BlizzardDisplay::Count(count));
        } else {
            map.insert(b.position, BlizzardDisplay::Direction(b.direction));
        }
    }

    map
}

fn draw_blizzards(blizzards: &Vec<Blizzard>, width: usize, height: usize) {
    let map = get_blizzard_map(blizzards);

    println!("S");
    for j in 0..height {

        for i in 0..width {
            let p = &Position { x: i as i32, y: j as i32};
            if let Some(display) = map.get(p) {
                match display {
                    BlizzardDisplay::Direction(d) => {
                        match d {
                            Direction::Up => print!("^"),
                            Direction::Down => print!("v"),
                            Direction::Right => print!(">"),
                            Direction::Left => print!("<"),
                        }
                    },
                    BlizzardDisplay::Count(n) => print!("{:?}", n),
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
    for _ in 0..(width-1) {
        print!(" ");
    }
    println!("E");
}

#[derive(Debug)]
enum BlizzardDisplay {
    Direction(Direction),
    Count(i32),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Blizzard {
    direction: Direction,
    position: Position,
}

impl Blizzard {
    fn sim(&mut self, width: u32, height: u32) {
        match self.direction {
            Direction::Up => {
                let other = Position { x: 0, y: -1 };
                let new_pos = self.position.add_wrapped(other, width as i32, height as i32);
                self.position = new_pos;
            }
            Direction::Down => {
                let other = Position { x: 0, y: 1 };
                let new_pos = self.position.add_wrapped(other, width as i32, height as i32);
                self.position = new_pos;
            }
            Direction::Right => {
                let other = Position { x: 1, y: 0 };
                let new_pos = self.position.add_wrapped(other, width as i32, height as i32);
                self.position = new_pos;
            }
            Direction::Left => {
                let other = Position { x: -1, y: 0 };
                let new_pos = self.position.add_wrapped(other, width as i32, height as i32);
                self.position = new_pos;
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn add(&mut self, other: Position) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }

    fn add_wrapped(&self, other: Position, width: i32, height: i32) -> Self {
        let x = (self.x + other.x).rem_euclid(width);
        let y = (self.y + other.y).rem_euclid(height);

        Position { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[cfg(test)]
mod tests {

    use super::*;

    const BASIC_INPUT_DAY_24: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn part_1_works() {
        assert_eq!(18, part_1(&BASIC_INPUT_DAY_24));
    }

    #[test]
    fn test_wrapping() {
        let p = Position { x: 0, y: 0 };

        let p2 = p.add_wrapped(Position { x: 1, y: 0 }, 4, 4);
        let expected = Position { x: 1, y: 0 };
        assert_eq!(expected, p2);

        let p3 = p2.add_wrapped(Position { x: 1, y: 0 }, 4, 4);
        let expected = Position { x: 2, y: 0 };
        assert_eq!(expected, p3);

        let p4 = p3.add_wrapped(Position { x: 1, y: 0 }, 4, 4);
        let expected = Position { x: 3, y: 0 };
        assert_eq!(expected, p4);

        let p5 = p4.add_wrapped(Position { x: 1, y: 0 }, 4, 4);
        let expected = Position { x: 0, y: 0 };
        assert_eq!(expected, p5);
    }
}
