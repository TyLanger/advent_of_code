use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    fs,
};

fn main() {
    let input = fs::read_to_string("./inputs/day_24_input.txt").unwrap();

    // 175 is too low
    // 301 is correct
    // println!("{}", part_1(&input));
    println!("{}", part_2(&input)); // 859
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

    let mut expedition_positions = HashSet::new();
    expedition_positions.insert(Position { x: 0, y: -1 }); // starting pos
    let end = Position {
        x: width as i32 - 1,
        y: height as i32,
    };
    println!("Looking for {}", end);

    let mut m = 0;
    loop {
        m += 1;
        println!();
        println!("Minute {}", m);
        simulate(&mut blizzards, width as u32, height as u32);
        draw_blizzards(&blizzards, width, height);
        expedition_positions =
            get_possible_positions(&blizzards, &expedition_positions, width, height);
        draw_possibilities(&expedition_positions, width, height);

        println!("Possible position count: {:?}", expedition_positions.len());
        // println!("{}", expedition_positions);
        // println!("{}", format!("{:?}", expedition_positions));
        // for pos in &expedition_positions {
        //     print!("{}, ", pos);
        // }
        // println!();

        if expedition_positions.contains(&end) {
            println!("Found end at minute: {:?}", m);
            println!("Looking for {}", end);
            return m;
        }
    }
}

fn part_2(input: &str) -> u32 {
    // do part 1
    // then clear possible positions
    // sim for start
    // clear
    // sim for end

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
    let start = Position { x: 0, y: -1 };
    let end = Position {
        x: width as i32 - 1,
        y: height as i32,
    };


    let mut expedition_positions = HashSet::new();
    expedition_positions.insert(start); // starting pos
    
    println!("Looking for {}", end);

    let mut m = 0;
    loop {
        m += 1;
        println!();
        println!("Minute {}", m);
        simulate(&mut blizzards, width as u32, height as u32);
        draw_blizzards(&blizzards, width, height);
        expedition_positions =
            get_possible_positions(&blizzards, &expedition_positions, width, height);
        draw_possibilities(&expedition_positions, width, height);

        println!("Possible position count: {:?}", expedition_positions.len());
        // println!("{}", expedition_positions);
        // println!("{}", format!("{:?}", expedition_positions));
        // for pos in &expedition_positions {
        //     print!("{}, ", pos);
        // }
        // println!();

        if expedition_positions.contains(&end) {
            println!("Found end at minute: {:?}", m);
            println!("Looking for {}", end);
            // return m;
            break;
        }
    }

    expedition_positions.clear();
    expedition_positions.insert(end);

    let mut back_to_start = 0;
    loop {
        back_to_start += 1;
        println!();
        println!("Minute {}", back_to_start);
        simulate(&mut blizzards, width as u32, height as u32);
        draw_blizzards(&blizzards, width, height);

        expedition_positions =
            get_possible_positions_reverse(&blizzards, &expedition_positions, width, height);
        draw_possibilities(&expedition_positions, width, height);
        println!("Possible position count: {:?}", expedition_positions.len());

        if expedition_positions.contains(&start) {
            println!("Found start at minute: {:?}", back_to_start);
            println!("Looking for {}", start);
            // return m;
            break;
        }

    }

    expedition_positions.clear();
    expedition_positions.insert(start);

    let mut back_to_end = 0;
    loop {
        back_to_end += 1;
        println!();
        println!("Minute {}", back_to_end);
        simulate(&mut blizzards, width as u32, height as u32);
        draw_blizzards(&blizzards, width, height);
        expedition_positions =
            get_possible_positions(&blizzards, &expedition_positions, width, height);
        draw_possibilities(&expedition_positions, width, height);

        println!("Possible position count: {:?}", expedition_positions.len());
        // println!("{}", expedition_positions);
        // println!("{}", format!("{:?}", expedition_positions));
        // for pos in &expedition_positions {
        //     print!("{}, ", pos);
        // }
        // println!();

        if expedition_positions.contains(&end) {
            println!("Found end end at minute: {:?}", back_to_end);
            println!("Looking for {}", end);
            // return m;
            break;
        }
    }

    m + back_to_start + back_to_end
}

fn get_possible_positions(
    blizzards: &Vec<Blizzard>,
    expedition_positions: &HashSet<Position>,
    width: usize,
    height: usize,
) -> HashSet<Position> {
    let mut set = HashSet::new();

    let blizz_set = get_blizzard_pos_set(blizzards);

    for p in expedition_positions {
        // check neighbours
        // are they blizzards?
        if !blizz_set.contains(p) {
            set.insert(*p);
        }
        for n in p.get_neighbours() {
            if n.x < 0 || n.x >= width as i32 || n.y < 0 || n.y >= height as i32 {
                // out of bounds

                // end is width -1, height
                if n.x == (width as i32 - 1) && n.y == height as i32 {
                    set.insert(n);
                    return set;
                }
                continue;
            }

            if !blizz_set.contains(&n) {
                set.insert(n);
            }
        }
    }

    set
}

// lazy solution
// same as the normal
// but short circuits at start(0,-1) instead of end(5,4)
fn get_possible_positions_reverse(
    blizzards: &Vec<Blizzard>,
    expedition_positions: &HashSet<Position>,
    width: usize,
    height: usize,
) -> HashSet<Position> {
    let mut set = HashSet::new();

    let blizz_set = get_blizzard_pos_set(blizzards);

    for p in expedition_positions {
        // check neighbours
        // are they blizzards?
        if !blizz_set.contains(p) {
            set.insert(*p);
        }
        for n in p.get_neighbours() {
            if n.x < 0 || n.x >= width as i32 || n.y < 0 || n.y >= height as i32 {
                // out of bounds

                // end is width -1, height
                // if n.x == (width as i32 - 1) && n.y == height as i32 {
                //     set.insert(n);
                //     return set;
                // }
                // start is 0, -1
                if n.x == 0 && n.y == -1 {
                    set.insert(n);
                    return set;
                }
                

                continue;
            }

            if !blizz_set.contains(&n) {
                set.insert(n);
            }
        }
    }

    set
}

fn simulate(blizzards: &mut Vec<Blizzard>, width: u32, height: u32) {
    for b in blizzards {
        b.sim(width, height);
    }
}

fn get_blizzard_pos_set(blizzards: &Vec<Blizzard>) -> HashSet<Position> {
    let mut set = HashSet::new();

    for b in blizzards {
        set.insert(b.position);
    }

    set
}

fn get_blizzard_map(blizzards: &Vec<Blizzard>) -> HashMap<Position, BlizzardDisplay> {
    let mut map: HashMap<Position, BlizzardDisplay> = HashMap::new();

    for &b in blizzards {
        let get = map.get(&b.position);
        if let Some(display) = get {
            let count = match display {
                BlizzardDisplay::Direction(_) => 2,
                BlizzardDisplay::Count(n) => n + 1,
            };
            map.insert(b.position, BlizzardDisplay::Count(count));
        } else {
            map.insert(b.position, BlizzardDisplay::Direction(b.direction));
        }
    }

    map
}

fn draw_possibilities(positions: &HashSet<Position>, width: usize, height: usize) {
    println!("E");
    for j in 0..height {
        for i in 0..width {
            let p = Position {
                x: i as i32,
                y: j as i32,
            };
            if positions.contains(&p) {
                print!("E");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn draw_blizzards(blizzards: &Vec<Blizzard>, width: usize, height: usize) {
    let map = get_blizzard_map(blizzards);

    println!("S");
    for j in 0..height {
        for i in 0..width {
            let p = &Position {
                x: i as i32,
                y: j as i32,
            };
            if let Some(display) = map.get(p) {
                match display {
                    BlizzardDisplay::Direction(d) => match d {
                        Direction::Up => print!("^"),
                        Direction::Down => print!("v"),
                        Direction::Right => print!(">"),
                        Direction::Left => print!("<"),
                    },
                    BlizzardDisplay::Count(n) => print!("{:?}", n),
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
    for _ in 0..(width - 1) {
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
                let new_pos = self
                    .position
                    .add_wrapped(other, width as i32, height as i32);
                self.position = new_pos;
            }
            Direction::Down => {
                let other = Position { x: 0, y: 1 };
                let new_pos = self
                    .position
                    .add_wrapped(other, width as i32, height as i32);
                self.position = new_pos;
            }
            Direction::Right => {
                let other = Position { x: 1, y: 0 };
                let new_pos = self
                    .position
                    .add_wrapped(other, width as i32, height as i32);
                self.position = new_pos;
            }
            Direction::Left => {
                let other = Position { x: -1, y: 0 };
                let new_pos = self
                    .position
                    .add_wrapped(other, width as i32, height as i32);
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

    fn get_neighbours(&self) -> Vec<Self> {
        let mut v = Vec::new();

        v.push(Position {
            x: self.x,
            y: self.y + 1,
        });
        v.push(Position {
            x: self.x,
            y: self.y - 1,
        });
        v.push(Position {
            x: self.x + 1,
            y: self.y,
        });
        v.push(Position {
            x: self.x - 1,
            y: self.y,
        });

        v
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
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
    fn part_2_works() {
        assert_eq!(54, part_2(&BASIC_INPUT_DAY_24));
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
