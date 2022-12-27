use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("./inputs/day_23_input.txt").unwrap();

    println!("{}", part_1(&input));
}

fn part_1(input: &str) -> u32 {
    // cellular automata basically
    // like game of life

    // simulate for 10 rounds
    // find the minimum bounding box
    // count ground tiles
    // which is the size - num_elves

    // elves have a pos
    // check nearby pos for other elves
    // look before moving
    // could be a hashmap
    // key is a point, value is how many elves wanna go there
    // only move into new pos if the value is 1(just you)

    // order
    // check 8 neighborus
    // if no other elves, stay put
    // else
    // try moving N, S, W, E
    // after each elf has chosen a desired direction,
    // each elf tries to move
    // they only move if no other elf wanted to move there

    // need to know own pos
    // check other elves' positions
    // remember their selection between deciding and moving steps
    // vec elves
    // hashset<pos> elf_positions - reset when moved
    // hashmap<pos, desired_pos> my_choice
    // hashmap<pos, count> collisions
    //

    let mut elves = Vec::new();

    // parse
    for (h, line) in input.lines().enumerate() {
        let split: Vec<&str> = line.split("").collect();
        // println!("split: {:?}", split);

        // "", ".", "#", ".", ""
        // blank at start and end
        // skip the first one and the last one doesn't matter (it doesn't match #)
        for (w, s) in split.iter().skip(1).enumerate() {
            if s == &"#" {
                let pos = Position {
                    x: w as i32,
                    y: h as i32,
                };
                let elf = Elf { pos };
                elves.push(elf);
                // println!("elf at {:?}", pos);
            }
        }
    }

    // println!("Elves: {:#?}", &elves[1]);
    println!("Start");
    display_elves(&elves);


    for i in 0..10 {
        let set = calc_elf_positions(&elves);
        let mut propose_move: HashMap<Position, u32> = HashMap::new();
        let mut my_move: HashMap<Position, Position> = HashMap::new();

        for e in &elves {
            // check if any elves nearby
            if any_nearby_elves(e.pos, &set) {
                // check above
                // then below, etc
                let open_dir = empty_direction(e.pos, &set);
                match open_dir {
                    Some(Direction::Up) => {
                        let up = Position {
                            x: e.pos.x,
                            y: e.pos.y - 1,
                        };
                        let hit = propose_move.get(&up);
                        if let Some(hit) = hit {
                            propose_move.insert(up, hit + 1);
                        } else {
                            propose_move.insert(up, 1);
                            my_move.insert(e.pos, up);
                        }
                    }
                    Some(Direction::Down) => {
                        let down = Position {
                            x: e.pos.x,
                            y: e.pos.y + 1,
                        };
                        let hit = propose_move.get(&down);
                        if let Some(hit) = hit {
                            propose_move.insert(down, hit + 1);
                        } else {
                            propose_move.insert(down, 1);
                            my_move.insert(e.pos, down);
                        }
                    }
                    Some(Direction::Left) => {
                        let left = Position {
                            x: e.pos.x - 1,
                            y: e.pos.y,
                        };
                        let hit = propose_move.get(&left);
                        if let Some(hit) = hit {
                            propose_move.insert(left, hit + 1);
                        } else {
                            propose_move.insert(left, 1);
                            my_move.insert(e.pos, left);
                        }
                    }
                    Some(Direction::Right) => {
                        let right = Position {
                            x: e.pos.x + 1,
                            y: e.pos.y,
                        };
                        let hit = propose_move.get(&right);
                        if let Some(hit) = hit {
                            propose_move.insert(right, hit + 1);
                        } else {
                            propose_move.insert(right, 1);
                            my_move.insert(e.pos, right);
                        }
                    }
                    None => {
                        // println!("Blocked in");
                    }
                }
            }
        }

        for e in elves.iter_mut() {
            let new_pos = my_move.get(&e.pos);
            if let Some(new_pos) = new_pos {
                if let Some(&count) = propose_move.get(&new_pos) {
                    // println!("count: {:?}", count);
                    if count == 1 {
                        // *e.pos = new_pos;
                        *e = Elf { pos: *new_pos };
                    }
                }
            }
        }
        println!("After round: {}", i);
        display_elves(&elves);

        // println!("Moved elves");
        // println!("Elves: {:#?}", &elves[1]);
        // println!("new: {:?}", my_move.get(&elves[0].pos));
    }

    // find the bounds
    let bounds = get_bounds(&elves);

    println!("Bounds: {:?}", bounds);
    // println!("Bounds: top: {}, bot: {}, left: {}, right: {}", top, bot, left, right);
    // height: 11
    // width: 12
    let height = bounds.bot - bounds.top;
    let width = bounds.right - bounds.left;
    println!("Height: {}, width: {}", height, width);


    println!("num elves: {:?}", &elves.len());
    display_elves(&elves);

    

    height as u32 * width as u32 - (elves.len() as u32)
}

fn display_elves(elves: &Vec<Elf>) {
    let bounds = get_bounds(elves);
    let height = bounds.bot - bounds.top;
    let width = bounds.right - bounds.left;

    let positions = calc_elf_positions(elves);

    println!();
    for h in 0..=height {
        for w in 0..=width {
            let pos = Position {
                x: w + bounds.left,
                y: h + bounds.top,
            };

            if positions.contains(&pos) {
                print!("#");
            } else  {
                print!(".");
            }
        }
        println!();
    }
}

fn get_bounds(elves: &Vec<Elf>) -> Bounds {
    let mut top = elves[0].pos.y;
    let mut bot = elves[0].pos.y;
    let mut left = elves[0].pos.x;
    let mut right = elves[0].pos.x;

    for e in elves {
        if e.pos.x > right {
            right = e.pos.x;
        }
        if e.pos.x < left {
            left = e.pos.x;
        }
        if e.pos.y > bot {
            bot = e.pos.y;
        }
        if e.pos.y < top {
            top = e.pos.y;
        }
    }

    Bounds {
        top,
        bot,
        left,
        right,
    }
}

fn calc_elf_positions(elves: &Vec<Elf>) -> HashSet<Position> {
    let mut set = HashSet::new();
    for e in elves {
        set.insert(e.pos);
    }
    set
}

fn any_nearby_elves(pos: Position, set: &HashSet<Position>) -> bool {
    let up_left = Position {
        x: pos.x - 1,
        y: pos.y - 1,
    };
    let up = Position {
        x: pos.x,
        y: pos.y - 1,
    };
    let up_right = Position {
        x: pos.x + 1,
        y: pos.y - 1,
    };

    let down = Position {
        x: pos.x,
        y: pos.y + 1,
    };
    let down_left = Position {
        x: pos.x - 1,
        y: pos.y + 1,
    };
    let down_right = Position {
        x: pos.x + 1,
        y: pos.y + 1,
    };

    let left = Position {
        x: pos.x - 1,
        y: pos.y,
    };
    let right = Position {
        x: pos.x + 1,
        y: pos.y,
    };

    set.contains(&up_left)
        || set.contains(&up)
        || set.contains(&up_right)
        || set.contains(&down_left)
        || set.contains(&down)
        || set.contains(&down_right)
        || set.contains(&left)
        || set.contains(&right)
}

fn empty_direction(pos: Position, set: &HashSet<Position>) -> Option<Direction> {
    let up_left = Position {
        x: pos.x - 1,
        y: pos.y - 1,
    };
    let up = Position {
        x: pos.x,
        y: pos.y - 1,
    };
    let up_right = Position {
        x: pos.x + 1,
        y: pos.y - 1,
    };
    if !(set.contains(&up_left) || set.contains(&up) || set.contains(&up_right)) {
        return Some(Direction::Up);
    }

    let down = Position {
        x: pos.x,
        y: pos.y + 1,
    };
    let down_left = Position {
        x: pos.x - 1,
        y: pos.y + 1,
    };
    let down_right = Position {
        x: pos.x + 1,
        y: pos.y + 1,
    };
    if !(set.contains(&down_left) || set.contains(&down) || set.contains(&down_right)) {
        return Some(Direction::Down);
    }

    let left = Position {
        x: pos.x - 1,
        y: pos.y,
    };
    if !(set.contains(&down_left) || set.contains(&left) || set.contains(&up_left)) {
        return Some(Direction::Left);
    }

    let right = Position {
        x: pos.x + 1,
        y: pos.y,
    };
    if !(set.contains(&down_right) || set.contains(&right) || set.contains(&up_right)) {
        return Some(Direction::Right);
    }

    None
}

#[derive(Debug)]
struct Bounds {
    top: i32,
    bot: i32,
    left: i32,
    right: i32,
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Elf {
    pos: Position,
}

#[cfg(test)]
mod tests {

    use super::*;

    const BASIC_INPUT_DAY_23: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

    #[test]
    fn part_1_works() {
        assert_eq!(110, part_1(&BASIC_INPUT_DAY_23));
    }

    #[test]
    fn nearby_elves_test() {
        let pos = Position { x: 0, y: 0 };
        let mut set = HashSet::new();

        // empty
        assert_eq!(false, any_nearby_elves(pos, &set));

        // right
        set.insert(Position { x: 1, y: 0 });
        assert_eq!(true, any_nearby_elves(pos, &set));

        // too far
        let mut set = HashSet::new();
        set.insert(Position { x: 2, y: 0 });
        assert_eq!(false, any_nearby_elves(pos, &set));

        // 8 neighbours
        let mut set = HashSet::new();
        set.insert(Position { x: -1, y: -1 });
        assert_eq!(true, any_nearby_elves(pos, &set));
        let mut set = HashSet::new();
        set.insert(Position { x: 0, y: -1 });
        assert_eq!(true, any_nearby_elves(pos, &set));
        let mut set = HashSet::new();
        set.insert(Position { x: 1, y: -1 });
        assert_eq!(true, any_nearby_elves(pos, &set));

        let mut set = HashSet::new();
        set.insert(Position { x: -1, y: 1 });
        assert_eq!(true, any_nearby_elves(pos, &set));
        let mut set = HashSet::new();
        set.insert(Position { x: 0, y: 1 });
        assert_eq!(true, any_nearby_elves(pos, &set));
        let mut set = HashSet::new();
        set.insert(Position { x: 1, y: 1 });
        assert_eq!(true, any_nearby_elves(pos, &set));

        let mut set = HashSet::new();
        set.insert(Position { x: -1, y: 0 });
        assert_eq!(true, any_nearby_elves(pos, &set));
        let mut set = HashSet::new();
        set.insert(Position { x: 1, y: 0 });
        assert_eq!(true, any_nearby_elves(pos, &set));
    }

    #[test]
    fn test_open_side() {
        let pos = Position { x: 0, y: 0 };
        let mut set = HashSet::new();
        // empty / default
        assert_eq!(Some(Direction::Up), empty_direction(pos, &set));

        // up with some neighbour
        set.insert(Position { x: 1, y: 0 });
        assert_eq!(Some(Direction::Up), empty_direction(pos, &set));
        // down when up blocked
        set.insert(Position { x: 1, y: -1 });
        assert_eq!(Some(Direction::Down), empty_direction(pos, &set));
        // left
        set.insert(Position { x: 1, y: 1 });
        assert_eq!(Some(Direction::Left), empty_direction(pos, &set));
        // none if all blocked
        set.insert(Position { x: -1, y: 1 });
        assert_eq!(None, empty_direction(pos, &set));

        // right as last resort
        let mut set = HashSet::new();
        set.insert(Position { x: -1, y: -1 });
        set.insert(Position { x: 0, y: 1 });
        assert_eq!(Some(Direction::Right), empty_direction(pos, &set));
    }
}
