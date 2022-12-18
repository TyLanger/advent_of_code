use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("./inputs/day_18_input.txt").unwrap();

    println!("{}", part_1(&input)); // 4302
    println!("{}", part_2(&input)); // 2492
}

fn part_1(input: &str) -> u32 {
    // 2800 cubes

    let cubes = parse_to_cubes(input);

    let mut cube_map = HashSet::new();
    for c in &cubes {
        cube_map.insert(c);
    }

    let mut open_faces = 0;
    for c in &cubes {
        let mut hit_count = 0;

        for n in c.get_neighbours(22, 22, 22) {
            if let Some(n) = n {
                if cube_map.contains(&n) {
                    hit_count += 1;
                }
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
    // start somewhere. then check neighbours. And their neighbours, etc.

    let cubes = parse_to_cubes(input);
    // println!("cubes len: {:?}", &cubes.len());

    let mut min_x = 1000;
    let mut min_y = 1000;
    let mut min_z = 1000;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;

    let mut cube_set: HashSet<&Cube> = HashSet::new();
    for c in &cubes {
        if c.x < min_x {
            min_x = c.x;
        } else if c.x > max_x {
            max_x = c.x;
        }
        if c.y < min_y {
            min_y = c.y;
        } else if c.y > max_y {
            max_y = c.y;
        }
        if c.z < min_z {
            min_z = c.z;
        } else if c.z > max_z {
            max_z = c.z;
        }

        cube_set.insert(c);
    }

    // 1 3 1 3 1 6
    // 0 21 0 21 0 20
    println!(
        "x: {} {}, y: {} {}, z: {} {}",
        min_x, max_x, min_y, max_y, min_z, max_z
    );

    // total cubes = 21 * 21 * 20
    // solid: cubes.len
    // air = total - solid

    // If I can find the interior air, I can set them all to cubes
    // and then the normal algorithm gets the right answer

    // flood fill from 0,0,0 (air)
    // fill with air
    // I now have a set of air and a set of blocks
    // in face count
    // only count a face if it's air
    // don't count if another block or nothing

    // flood fill
    // 0, 0, 0 is air (I checked)
    let current_cube = Cube { x: 0, y: 0, z: 0 };
    let mut air: HashSet<Cube> = HashSet::new();
    air.insert(current_cube);

    let mut queue = Vec::new();
    queue.push(current_cube);

    while !queue.is_empty() {
        let current = queue.pop().unwrap();
        // max + 1
        // hack
        // The blocks probably cut you off if you flood fill from 0,0,0
        // so go out of bounds a bit to get around this block
        for n in current.get_neighbours(max_x + 1, max_y + 1, max_z + 1) {
            // is a neighbour in air?
            // is it a block?
            if let Some(n) = n {
                if !within_bounds(&n, max_x + 2, max_y + 2, max_z + 2) {
                    continue;
                }

                if cube_set.contains(&n) {
                    // it's a block
                    continue;
                }

                if air.insert(n) {
                    // was new
                    queue.push(n);
                }
            }
        }
    }

    // let num_cubes = cube_set.len();
    // let num_air = air.len();
    // let space = (max_x - min_x + 2) * (max_y - min_y + 2) * (max_z - min_z + 2);
    // println!("num cubes: {:?}", num_cubes);
    // println!("air set: {:?}", num_air);
    // println!("cubes + air: {:?}", num_cubes + num_air);
    // println!("space: {:?}", space);
    // println!("difference: {:?}", space as usize - (num_cubes + num_air));

    // cubes + air < space
    // if no cubes, air == 54, space == 54
    // so floodfill should be correct

    let mut open_faces = 0;
    for c in &cubes {
        let mut air_face_count = 0;

        for n in c.get_neighbours(max_x, max_y, max_z) {
            if let Some(n) = n {
                if air.contains(&n) {
                    // air
                    // good face
                    air_face_count += 1;
                }
            } else {
                air_face_count += 1;
            }

            // faces that count:
            // air
            // or outside bounds
        }
        open_faces += air_face_count;
    }

    // 2473 is too low

    open_faces
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

fn within_bounds(cube: &Cube, x_max: u32, y_max: u32, z_max: u32) -> bool {
    cube.x < x_max && cube.y < y_max && cube.z < z_max
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Cube {
    x: u32,
    y: u32,
    z: u32,
}

impl Cube {
    fn get_neighbours(&self, max_x: u32, max_y: u32, max_z: u32) -> Vec<Option<Cube>> {
        let mut v = Vec::new();

        //up
        if self.z < max_z {
            let up = Cube {
                x: self.x,
                y: self.y,
                z: self.z + 1,
            };
            v.push(Some(up));
        } else {
            v.push(None);
        }

        // right
        if self.x < max_x {
            let right = Cube {
                x: self.x + 1,
                y: self.y,
                z: self.z,
            };
            v.push(Some(right));
        } else {
            v.push(None);
        }

        //forward
        if self.y < max_y {
            let forward = Cube {
                x: self.x,
                y: self.y + 1,
                z: self.z,
            };
            v.push(Some(forward));
        } else {
            v.push(None);
        }

        // down
        if self.z > 0 {
            let down = Cube {
                x: self.x,
                y: self.y,
                z: self.z - 1,
            };
            v.push(Some(down));
        } else {
            v.push(None);
        }

        // left
        if self.x > 0 {
            let left = Cube {
                x: self.x - 1,
                y: self.y,
                z: self.z,
            };
            v.push(Some(left));
        } else {
            v.push(None);
        }

        // back
        if self.y > 0 {
            let back = Cube {
                x: self.x,
                y: self.y - 1,
                z: self.z,
            };
            v.push(Some(back));
        } else {
            v.push(None);
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
        let cube = Cube { x: 2, y: 2, z: 2 };

        let neighbours = cube.get_neighbours(3, 3, 3);
        assert_eq!(6, neighbours.len());

        let expected = vec![
            Some(Cube { x: 2, y: 2, z: 3 }),
            Some(Cube { x: 3, y: 2, z: 2 }),
            Some(Cube { x: 2, y: 3, z: 2 }),
            Some(Cube { x: 2, y: 2, z: 1 }),
            Some(Cube { x: 1, y: 2, z: 2 }),
            Some(Cube { x: 2, y: 1, z: 2 }),
        ];
        assert_eq!(expected, neighbours);

        let cube = Cube { x: 0, y: 0, z: 0 };

        let neighbours = cube.get_neighbours(3, 3, 3);
        assert_eq!(6, neighbours.len());
        let expected = vec![
            Some(Cube { x: 0, y: 0, z: 1 }),
            Some(Cube { x: 1, y: 0, z: 0 }),
            Some(Cube { x: 0, y: 1, z: 0 }),
            None,
            None,
            None,
        ];
        assert_eq!(expected, neighbours);
    }
}
