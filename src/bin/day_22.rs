use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./inputs/day_22_input.txt").unwrap();

    println!("{}", part_1(&input));
}

fn part_1(input: &str) -> usize {
    // start facing right
    // move forward the number or until hitting a wall
    // the turn
    // repeat

    // wrap around somehow

    // start on the leftmost tile of the top row

    // is it floor, wall, or blank?
    let mut rows = Vec::new();
    // // holds the edge indicies
    // let edges = Vec::new();
    // let vert_edges = Vec::new();
    // let in_split = input.split_once("\n\r\n\r").unwrap();
    // let map = in_split.0;
    // let commands =  in_split.1;
    // println!("map: {:?}", map);
    // println!("commands: {:?}", commands);
    let num_lines = input.lines().count();
    let mut width = 0;
    let height = num_lines - 2;

    for line in input.lines().take(height) {
        if line.len() > width {
            width = line.len();
        }
    }

    for line in input.lines().take(height) {
        let split: Vec<&str> = line.split("").skip(1).collect();
        // println!("{:?}", split);
        // ["", " ", " ", ".", "#", ""]
        // ["", " ", " ", ".", "#", ".", ".", ""]
        // some are longer

        let mut tiles = Vec::new();
        for i in 0..width {
            if i < (split.len() - 1) {
                match split[i] {
                    " " => tiles.push(Tile::Blank),
                    "." => tiles.push(Tile::Floor),
                    "#" => tiles.push(Tile::Wall),
                    _ => panic!("no matches"),
                }
            } else {
                tiles.push(Tile::Blank);
            }
        }
        // println!("Tiles: {:?}", tiles);
        rows.push(tiles);
    }

    let mut commands: Vec<Command> = Vec::new();
    for s in input.lines().last().unwrap().split_inclusive(&['L', 'R']) {
        // println!("s: {:?}", s);
        // kept at the end of the str
        // 50R, 23L, 21
        // last is just a num
        let split = s.split_at(s.len() - 1);
        if let Ok(val) = split.0.parse::<u32>() {
            commands.push(Command::Move(val));
        } else {
            // last is just a num
            if let Ok(val) = s.parse() {
                commands.push(Command::Move(val));
                break;
            }
        }

        if split.1 == "R" {
            commands.push(Command::Rotate(Turn::Right));
        } else if split.1 == "L" {
            commands.push(Command::Rotate(Turn::Left));
        }
    }
    // println!("Commands: {:?}", commands);

    let mut x = 0;
    let mut y = 0;
    // get starting pos
    for (i, item) in rows[0].iter().enumerate() {
        if item == &Tile::Floor {
            x = i;
            break;
        }
    }

    let mut facing = 0;
    // 0 is right, 1 is down, 2 is left, 3 is up

    println!("Start: ({}, {})", x, y);
    let mut visited = HashMap::new();

    for c in commands {
        // println!("Facing: {}", facing);
        // println!("C: {:?}", c);
        match c {
            Command::Move(val) => {
                for _i in 0..val {
                    if !visited.contains_key(&(x, y)) {
                        // if !exist
                        // only track the first direction
                        // still can't follow the path when it goes from the top to bottom
                        // it's too big
                        visited.insert((x, y), facing);
                    }

                    (x, y) = get_new_pos(x, y, width, height, facing, &rows);
                    // println!("({}, {})", x, y);
                }
            }
            Command::Rotate(dir) => match dir {
                Turn::Left => facing = (facing + 3) % 4,
                Turn::Right => facing = (facing + 1) % 4,
            },
        }
    }
    // last one 10 maps to @ for the end
    visited.insert((x, y), 10);

    println!("Visual");
    // visual
    for (r, row) in rows.iter().enumerate() {
        for (c, item) in row.iter().enumerate() {
            match item {
                Tile::Floor => {
                    if let Some(fac) = visited.get(&(c, r)) {
                        match fac {
                            0 => print!(">"),
                            1 => print!("v"),
                            2 => print!("<"),
                            3 => print!("^"),
                            10 => print!("@"),
                            _ => print!("!"),
                        }
                    } else {
                        print!(".");
                    }
                }
                Tile::Wall => print!("#"),
                Tile::Blank => print!(" "),
            }
        }
        println!();
    }

    let row = y + 1;
    let col = x + 1;
    let dir = facing;
    println!("End: row: {}, col: {}, dir: {}", row, col, dir);
    // 114004 is too high
    // end: 114, 1, 0
    // 58208 is too low
    // end: 58, 52, 0
    1000 * row + 4 * col + dir as usize
}

fn get_new_pos(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    facing: u32,
    rows: &Vec<Vec<Tile>>,
) -> (usize, usize) {
    let new_x;
    let new_y;

    match facing {
        // right
        0 => {
            new_x = (x + 1).rem_euclid(width);
            new_y = y;
            match rows[new_y][new_x] {
                Tile::Blank => {
                    // wrap around

                    for (i, item) in rows[y].iter().enumerate() {
                        if item != &Tile::Blank {
                            match item {
                                Tile::Floor => {
                                    return (i, y);
                                }
                                Tile::Wall => {
                                    return (x, y);
                                }
                                Tile::Blank => todo!(),
                            }
                        }
                    }
                }
                Tile::Floor => {
                    return (new_x, new_y);
                }
                Tile::Wall => {
                    // finish
                    return (x, y);
                }
            }
        }
        // down
        1 => {
            new_x = x; // (x+1).rem_euclid(width);
            new_y = (y + 1).rem_euclid(height);
            match rows[new_y][new_x] {
                Tile::Blank => {
                    // wrap around
                    for i in 0..height {
                        if Tile::Blank != rows[i][x] {
                            match rows[i][x] {
                                Tile::Floor => {
                                    return (x, i);
                                }
                                Tile::Wall => {
                                    return (x, y);
                                }
                                Tile::Blank => todo!(),
                            }
                        }
                    }
                }
                Tile::Floor => {
                    return (new_x, new_y);
                }
                Tile::Wall => {
                    // finish
                    return (x, y);
                }
            }
        }
        // left
        2 => {
            new_x = (x as isize - 1).rem_euclid(width as isize) as usize;
            new_y = y;
            match rows[new_y][new_x] {
                Tile::Blank => {
                    // find the rightmost tile
                    for (i, item) in rows[y].iter().rev().enumerate() {
                        if item != &Tile::Blank {
                            match item {
                                Tile::Floor => {
                                    return ((width - i) - 1, y);
                                }
                                Tile::Wall => {
                                    return (x, y);
                                }
                                Tile::Blank => todo!(),
                            }
                        }
                    }
                }
                Tile::Floor => {
                    return (new_x, new_y);
                }
                Tile::Wall => {
                    // finish
                    return (x, y);
                }
            }
        }
        // up
        3 => {
            new_x = x; // (x+1).rem_euclid(width);
            new_y = (y as isize - 1).rem_euclid(height as isize) as usize;
            match rows[new_y][new_x] {
                Tile::Blank => {
                    // wrap around
                    for i in 0..height {
                        if Tile::Blank != rows[(height - i) - 1][x] {
                            match rows[(height - i) - 1][x] {
                                Tile::Floor => {
                                    return (x, (height - i) - 1);
                                }
                                Tile::Wall => {
                                    return (x, y);
                                }
                                Tile::Blank => todo!(),
                            }
                        }
                    }
                }
                Tile::Floor => {
                    return (new_x, new_y);
                }
                Tile::Wall => {
                    // finish
                    return (x, y);
                }
            }
        }
        _ => panic!("wrong facing"),
    }

    println!("Dropped out end");
    (x, y)
}

// fn get_next_pos(x: usize, y: usize, facing: u32) -> (usize, usize) {
//     match facing {
//         0 => {
//             if
//             (x+1, y)
//         },
//         1 => (x, y-1),
//         2 => (x-1, y),
//         3 => (x, y+1),
//         _ => panic!("wrong facing"),
//     }
// }

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Floor,
    Wall,
    Blank,
}

#[derive(Debug)]
enum Command {
    Move(u32),
    Rotate(Turn),
}

#[derive(Debug)]
enum Turn {
    Left,
    Right,
}

#[cfg(test)]
mod tests {

    use super::*;

    const BASIC_INPUT_DAY_22: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    const BASIC_INPUT_DAY_22_B: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R4R3R10";

    const BASIC_INPUT_DAY_22_C: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L3L3";

    const BASIC_INPUT_DAY_22_D: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R3R3";

    const BASIC_INPUT_DAY_22_E: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R2L10";

    #[test]
    fn part_1_works() {
        assert_eq!(6032, part_1(&BASIC_INPUT_DAY_22));
    }

    #[test]
    fn part_1_works_b_input() {
        // 8,8
        assert_eq!(8035, part_1(&BASIC_INPUT_DAY_22_B));
        // 2,7
        assert_eq!(7011, part_1(&BASIC_INPUT_DAY_22_C));
        // go off left
        assert_eq!(4050, part_1(&BASIC_INPUT_DAY_22_D));
        // go off edge right and hit a wall
        assert_eq!(3048, part_1(&BASIC_INPUT_DAY_22_E));
    }

    #[test]
    fn test_simple_cases() {
        let input = "...
...
...

1";
        part_1(input);
        // what cases do I need?
        // walk right into blank
        // walk right into wall
        // is the problem it's not counting correctly when it wraps?
        // and it's just hard to tell bc it usually hits a wall?
        // maybe double wrapping is an issue?
        // maybe there's a 0 or 1 move that's broken

        // I should extract some functions to make it easier to test
    }

    #[test]
    fn test_rev() {
        let v = vec![5, 6, 7, 8];
        for (i, a) in v.iter().rev().enumerate() {
            println!("i: {}, a: {}", i, a);
        }
    }
}
