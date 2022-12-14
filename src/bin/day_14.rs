use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/day_14_input.txt").unwrap();

    println!("{}", part_1(&input));
}

fn part_1(_input: &str) -> u32 {
    // test input
    // x: 494-503, y: 4-9
    // real input:
    // x: 492 - 562, y: 13 - 173

    simulate_sand_count(_input, 492, 562, 173)

    
}

fn simulate_sand_count(input: &str, min_x: usize, max_x: usize, max_y: usize) -> u32 {
    let lines = input.lines();

    let width = max_x - min_x + 1;
    let height = max_y + 1;
    let mut grid = get_empty_material_grid(width, height);

    grid[0][500 - min_x] = Material::Source;

    for line in lines {
        let points_strings = line.split(" -> ");
        let points: Vec<Point> = points_strings.map(|x| Point::from_str(x, min_x)).collect();
        // println!("{:?}", points);

        for i in 0..(points.len() - 1) {
            fill_rocks(&mut grid, points[i], points[i + 1]);
        }
    }

    // display_grid(&grid);

    let mut count = drop_sand(&mut grid);

    count += drop_sand(&mut grid);
    count += drop_sand(&mut grid);

    count
    
}

fn drop_sand(grid: &mut Vec<Vec<Material>>) -> u32 {
    let mut source_x = 0;
    let width = grid[0].len();

    for i in 0..width {
        if grid[0][i] == Material::Source {
            source_x = i;
        }
    }
    let source_y = 0;

    let mut sand_count = 0;

    // println!("Source: {}, {}", source_x, source_y);
    let mut x = source_x;
    let mut y = source_y;
    'outer: for _ in 0..300 {
        x = source_x;
        y = source_y;

        loop {
            if y + 1 >= grid.len() {
                // fall out bottom
                println!("Fell out bottom");
                break 'outer;
            }
            if x < width && y < grid.len() {
                // within bounds
                match grid[y + 1][x] {
                    Material::Air => {
                        y += 1;
                    }
                    Material::Rock | Material::Sand => {
                        if x == 0 {
                            // ??
                            // fall off the edge
                            println!("Fell out left");

                            break;
                        }
                        match grid[y + 1][x - 1] {
                            Material::Air => {
                                y += 1;
                                x -= 1;
                                continue;
                            }
                            _ => {
                                if x == width {
                                    // fall off the right side
                                    println!("Fell out right");

                                    break;
                                }
                                match grid[y + 1][x + 1] {
                                    Material::Air => {
                                        y += 1;
                                        x += 1;
                                        continue;
                                    }
                                    _ => {}
                                }
                            }
                        }

                        // println!("Hit something");
                        grid[y][x] = Material::Sand;
                        sand_count += 1;
                        break;
                    }
                    Material::Source => todo!(),
                }
            }
        }
    }
    display_grid(&grid);
    sand_count
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn from_str(input: &str, x_decrement: usize) -> Self {
        let split = input.split_once(',');
        Point {
            x: split.unwrap().0.parse::<usize>().unwrap() - x_decrement,
            y: split.unwrap().1.parse::<usize>().unwrap(),
        }
    }

    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Material {
    Air,
    Rock,
    Sand,
    Source,
}

fn get_empty_material_grid(width: usize, height: usize) -> Vec<Vec<Material>> {
    let mut v = Vec::new();
    for i in 0..height {
        let vh = vec![Material::Air; width];
        v.push(vh);
    }

    v
}

fn fill_rocks(grid: &mut Vec<Vec<Material>>, start: Point, end: Point) {
    // grid[start.y][start.x] = Material::Rock;
    // grid[end.y][end.x] = Material::Rock;
    // println!("{:?} -> {:?}", start, end);

    let mut min_x = 0;
    let mut max_x = 0;
    if start.x < end.x {
        min_x = start.x;
        max_x = end.x;
    } else {
        min_x = end.x;
        max_x = start.x;
    }

    let mut min_y = 0;
    let mut max_y = 0;
    if start.y < end.y {
        min_y = start.y;
        max_y = end.y;
    } else {
        min_y = end.y;
        max_y = start.y;
    }

    for i in min_x..=max_x {
        for j in min_y..=max_y {
            grid[j][i] = Material::Rock;
        }
    }
}

fn display_grid(grid: &Vec<Vec<Material>>) {
    for row in grid {
        let mut row_out = "".to_string();
        for item in row {
            let letter = match item {
                Material::Air => ".",
                Material::Rock => "#",
                Material::Sand => "O",
                Material::Source => "+",
            };
            row_out = format!("{}{}", row_out, letter);
        }
        println!("{}", row_out);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const BASIC_INPUT_DAY_14: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn part_1_works() {
        // test input
        // x: 494-503, y: 4-9
        assert_eq!(24, simulate_sand_count(&BASIC_INPUT_DAY_14, 494, 503, 9));
    }

    #[test]
    fn point_from_str() {
        let input = "498,4";

        assert_eq!(Point::new(0, 4), Point::from_str(input, 498));
    }

    #[test]
    fn line_fills_in_rock() {
        let mut v = get_empty_material_grid(3, 3);
        let start = Point::new(0, 1);
        let end = Point::new(2, 1);

        fill_rocks(&mut v, start, end);

        display_grid(&v);

        assert_eq!(
            vec![
                vec![Material::Air, Material::Air, Material::Air],
                vec![Material::Rock, Material::Rock, Material::Rock],
                vec![Material::Air, Material::Air, Material::Air]
            ],
            v
        );

        // does it work backwards?
        let mut v = get_empty_material_grid(3, 3);
        let start = Point::new(0, 1);
        let end = Point::new(2, 1);

        fill_rocks(&mut v, end, start);

        display_grid(&v);

        assert_eq!(
            vec![
                vec![Material::Air, Material::Air, Material::Air],
                vec![Material::Rock, Material::Rock, Material::Rock],
                vec![Material::Air, Material::Air, Material::Air]
            ],
            v
        );

        // up and down
        let mut v = get_empty_material_grid(3, 3);
        let start = Point::new(1, 0);
        let end = Point::new(1, 2);

        fill_rocks(&mut v, start, end);

        display_grid(&v);

        assert_eq!(
            vec![
                vec![Material::Air, Material::Rock, Material::Air],
                vec![Material::Air, Material::Rock, Material::Air],
                vec![Material::Air, Material::Rock, Material::Air]
            ],
            v
        );

        // does it work backwards?
        let mut v = get_empty_material_grid(3, 3);
        let start = Point::new(1, 0);
        let end = Point::new(1, 2);

        fill_rocks(&mut v, end, start);

        display_grid(&v);

        assert_eq!(
            vec![
                vec![Material::Air, Material::Rock, Material::Air],
                vec![Material::Air, Material::Rock, Material::Air],
                vec![Material::Air, Material::Rock, Material::Air]
            ],
            v
        );
    }

    #[test]
    fn make_grid_just_big_enough() {
        // test input
        // x: 494-503, y: 4-9
        // real input:
        // x: 492 - 562, y: 13 - 173

        let min_x = 494;
        let max_x = 503;
        let min_y = 0;
        let max_y = 9;

        let width = max_x - min_x;
        let height = max_y - min_y;
        println!("{}, {}", width, height);

        let grid = get_empty_material_grid(width, height);
        display_grid(&grid);
    }
}
