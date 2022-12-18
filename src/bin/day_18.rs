use std::{fs, collections::{hash_map, HashSet}};

fn main() {
    let input = fs::read_to_string("./inputs/day_18_input.txt").unwrap();

    println!("{}", part_1(&input));
}

fn part_1(input: &str) -> u32 {
    // 2800 cubes

    let cubes = parse_to_cubes(input);
    // println!("cubes len: {:?}", &cubes.len());

    let mut cube_map = HashSet::new();
    for c in &cubes {
        cube_map.insert(c);
    }

    let mut open_faces = 0;
    for c in &cubes {
        let mut hit_count = 0;

        for n in c.get_neighbours() {
            if cube_map.contains(&n) {
                hit_count += 1;
            }
        }
        open_faces += 6 - hit_count;
    }


    open_faces
}

fn part_2(input: &str) -> u32 {

    // get outer faces
    // ignore interior faces

    // how do I find air pockets?
    // do I need an iterator or something to move around and explore the space?

    // raycast from each face
    // if it hits the edge on any face, it's open air
    // if all raycasts hit a block,
    // need to check for overhangs?

    // 3D floodfill
    // i don't know how

    99
}

fn parse_to_cubes(input: &str) -> Vec<Cube> {
    let mut v = Vec::new();
    for line in input.lines() {
        let split: Vec<&str> = line.split(',').collect();
        let cube = Cube {
            x: split[0].parse().unwrap(),
            y: split[1].parse().unwrap(),
            z: split[2].parse().unwrap(),
        };
        v.push(cube);
    }
    v
}



#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Cube {
    x: u32,
    y: u32,
    z: u32,
}

impl Cube {
    fn get_neighbours(&self) -> Vec<Cube> {
        let mut v = Vec::new();

        //up
        let up = Cube {
            x: self.x,
            y: self.y,
            z: self.z + 1,
        };
        v.push(up);
        // right
        let right = Cube {
            x: self.x + 1,
            y: self.y,
            z: self.z,
        };
        v.push(right);
        //forward
        let forward = Cube {
            x: self.x,
            y: self.y + 1,
            z: self.z,
        };
        v.push(forward);

        // down
        if self.z > 0 {
            let down = Cube {
                x: self.x,
                y: self.y,
                z: self.z - 1,
            };
            v.push(down);
        }

        // left
        if self.x > 0 {
            let left = Cube {
                x: self.x - 1,
                y: self.y,
                z: self.z,
            };
            v.push(left);
        }

        // back
        if self.y > 0 {
            let back = Cube {
                x: self.x,
                y: self.y - 1,
                z: self.z,
            };
            v.push(back);
        }

        v
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const BASIC_INPUT_DAY_18: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn part_1_works() {
        assert_eq!(64, part_1(&BASIC_INPUT_DAY_18));
    }

    #[test]
    fn part_2_works() {
        assert_eq!(58, part_2(&BASIC_INPUT_DAY_18));
    }

    #[test]
    fn test_cube_parse() {
        let cubes = parse_to_cubes(&BASIC_INPUT_DAY_18);
        let expect = Cube { x: 2, y: 2, z: 2 };
        assert_eq!(expect, cubes[0]);
    }

    #[test]
    fn test_neighbours() {
        let cube = Cube {x: 2, y: 2, z: 2};

        let neighbours = cube.get_neighbours();
        assert_eq!(6, neighbours.len());

        let expected = vec![
            Cube {x: 2, y: 2, z: 3},
            Cube {x: 3, y: 2, z: 2},
            Cube {x: 2, y: 3, z: 2},
            Cube {x: 2, y: 2, z: 1},
            Cube {x: 1, y: 2, z: 2},
            Cube {x: 2, y: 1, z: 2},
        ];
        assert_eq!(expected, neighbours);

        let cube = Cube {x: 0, y: 0, z: 0};

        let neighbours = cube.get_neighbours();
        assert_eq!(3, neighbours.len());
        let expected = vec![
            Cube {x: 0, y: 0, z: 1},
            Cube {x: 1, y: 0, z: 0},
            Cube {x: 0, y: 1, z: 0},
        ];
        assert_eq!(expected, neighbours);
    }
}
