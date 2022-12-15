use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("./inputs/day_15_input.txt").unwrap();

    println!("{}", part_1(&input)); // 5112034
    // part 1 takes a while to run
    // is that correct?
}

fn part_1(input: &str) -> usize {
    get_num_positions_at_row(input, 2_000_000)
}

fn get_num_positions_at_row(input: &str, row: usize) -> usize {
    // I know a sensor position
    // and its nearest beacon
    // can fill in everything the same distance from that sensor

    // only really need the one row
    // and then to try to calculate where the other stuff intercepts it
    // get a sensor and dist
    // does that dist intersect the row?
    // fill in those spaces

    let mut hits: HashSet<Point> = HashSet::new();
    let mut beacon_points: HashSet<Point> = HashSet::new();

    // parse to points
    for line in input.lines() {
        let split_arr: Vec<&str> = line.split(&['=', ',', ':']).collect();
        // println!("line: {:?}", split_arr);
        // for item in line.split(&['=',',',':']) {
        //     if let Ok(num) = item.trim().parse::<i32>() {

        //     }
        // }

        let x1 = split_arr[1].parse::<i32>().unwrap();
        let y1 = split_arr[3].parse::<i32>().unwrap();
        let x2 = split_arr[5].parse::<i32>().unwrap();
        let y2 = split_arr[7].parse::<i32>().unwrap();

        let p1 = Point { x: x1, y: y1 };
        let p2 = Point { x: x2, y: y2 };

        let dist = p1.get_distance(&p2);
        beacon_points.insert(p2);

        // println!("p1: {:?}, p2: {:?} dist: {:?}", &p1, &p2, dist);
        // println!("Intersect: {}", intersects_row(&p1, dist, row));

        if intersects_row(&p1, dist, row) {
            // now how many times does it intersect?
            // which points intersect?
            let v = intersection_points(&p1, dist, row);
            for item in v {
                hits.insert(item);
            }
        }
    }

    // let mut hits: HashSet<Point> = HashSet::new();

    // println!("hits: {:#?}", hits);
    // let mut v_hits: Vec<&Point> = hits.iter().collect();
    // v_hits.sort_by(|&a, &b| a.x.cmp(&b.x));
    // println!("v hits: {:#?}", &v_hits);

    // want hits without whatever is in beacon_points
    hits.difference(&beacon_points).count()


    // hits.len()
}

fn intersects_row(p: &Point, dist: u32, row: usize) -> bool {
    // p { 8, 7}
    // dist: 9
    // intersects rows:
    // -2 to 16
    // y +- dist

    let min = p.y - dist as i32;
    let max = p.y + dist as i32;

    let i_row = row as i32;

    min <= i_row && i_row <= max
}

fn intersection_points(p: &Point, dist: u32, row: usize) -> Vec<Point> {
    let mut v = Vec::new();

    let i_row = row as i32;
    let row_dist = i_row.abs_diff(p.y);

    // when row dist is 0, push 2 * dist items + 1
    // when row dist is == dist, push 1 item

    // 15 - 7 = 8
    // dist - 8 = 1
    // 16 - 7 = 9
    // 9 - 9 = 0
    let half_width = dist - row_dist;
    // println!("half width: {}", half_width);

    // all points will have the same row
    // 2x + 1
    // can insert the center +1 case
    let new_point = Point { x: p.x, y: i_row };
    v.push(new_point);

    // loop to get each side

    for i in 1..=half_width {
        let point = Point {
            x: p.x - i as i32,
            y: i_row,
        };
        v.push(point);
    }
    for i in 1..=half_width {
        let point = Point {
            x: p.x + i as i32,
            y: i_row,
        };
        v.push(point);
    }

    v
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn get_distance(&self, other: &Self) -> u32 {
        let x_diff = self.x.abs_diff(other.x);
        let y_diff = self.y.abs_diff(other.y);

        x_diff + y_diff
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const BASIC_INPUT_DAY_15: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    // #[ignore = "not ready"]
    fn part_1_works() {
        assert_eq!(26, get_num_positions_at_row(&BASIC_INPUT_DAY_15, 10));
    }

    #[test]
    fn manhattan_distance() {
        let p1 = Point { x: 8, y: 7 };
        let p2 = Point { x: 2, y: 10 };

        assert_eq!(9, p1.get_distance(&p2));

        let p1 = Point { x: -8, y: 7 };
        let p2 = Point { x: 2, y: -10 };

        assert_eq!(27, p1.get_distance(&p2));
    }

    #[test]
    fn test_row_intersection() {
        let p = Point { x: 8, y: 7 };
        assert_eq!(true, intersects_row(&p, 9, 10));

        let p = Point { x: 8, y: 7 };
        assert_eq!(true, intersects_row(&p, 9, 16));

        let p = Point { x: 8, y: 7 };
        assert_eq!(false, intersects_row(&p, 1, 10));

        let p = Point { x: -10, y: 7 };
        assert_eq!(true, intersects_row(&p, 3, 10));
    }

    #[test]
    fn test_intersection_points() {
        let p = Point { x: 8, y: 7 };
        let expected = vec![Point { x: 8, y: 16 }];
        assert_eq!(expected, intersection_points(&p, 9, 16));

        let p = Point { x: 8, y: 7 };
        let expected = vec![
            Point { x: 8, y: 15 },
            Point { x: 7, y: 15 },
            Point { x: 9, y: 15 },
        ];
        assert_eq!(expected, intersection_points(&p, 9, 15));

        let p = Point { x: 8, y: 7 };
        let expected = vec![
            Point { x: 8, y: 14 },
            Point { x: 7, y: 14 },
            Point { x: 6, y: 14 },
            Point { x: 9, y: 14 },
            Point { x: 10, y: 14 },
        ];
        assert_eq!(expected, intersection_points(&p, 9, 14));


        let p = Point { x: 8, y: 7 };
        assert_eq!(13, intersection_points(&p, 9, 10).len());


    }
}
