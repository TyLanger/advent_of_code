use std::{collections::HashSet, fs, time::SystemTime};

fn main() {
    let input = fs::read_to_string("./inputs/day_15_input.txt").unwrap();

    let now = SystemTime::now();
    println!("Start: {:?}", now);

    println!("{}", part_1(&input)); // 5112034
                                    // part 1 takes a while to run
                                    // is that correct?
    let mid = SystemTime::now();
    let diff = mid.duration_since(now);
    if let Ok(d) = diff {
        println!("Start to mid in secs: {:?}", d.as_secs());
    }
    println!("After Part 1: {:?}", mid);
    println!("{}", part_2(&input)); // 13,172,087,230,812
    // 100
    // 100
    // ????
    // 300s without stopping???
    // should 4m * 4m * 26
    // take that long???

    // does i ever make it to 10_000??

    // maybe it is too long

    // running i 0..1
    // j 0..4m
    // takes 5s
    // 5s x 4m = 5555 hours???

    let end = SystemTime::now();
    let diff = end.duration_since(now);
    if let Ok(d) = diff {
        println!("Start to end in secs: {:?}", d.as_secs());
    }
    println!("End: {:?}", end);


    // Start: SystemTime { intervals: 133155661742588908 }
    // 5112034
    // Start to mid in secs: 11
    // After Part 1: SystemTime { intervals: 133155661852698865 }
    // ======= point: Point { x: 3293021, y: 3230812 }
    // 3293021
    // Start to end in secs: 13
    // End: SystemTime { intervals: 133155661874088686 }
    // real answer: 
    // 13,172,087,230,812
    // hacked to work with i32
    // changed it to u64 and it can output fine
}

fn part_1(input: &str) -> usize {
    get_num_positions_at_row(input, 2_000_000)
}

fn part_2(input: &str) -> usize {
    // find the 1 spot it could be
    // 14, 11 in the example

    //

    get_distress_frequency(input, 4_000_000)
}

fn get_distress_frequency(input: &str, upper_bounds: usize) -> usize {
    // loop over x..upper_bounds
    // loop over y..upper_bounds
    // check the distance to each sensor
    // is it within range?
    // move on until we find the 1 position
    // where it is out of range of each sensor

    // calculate each sensor distance
    let mut sensors: Vec<Point> = Vec::new();
    let mut distances: Vec<u32> = Vec::new();

    for line in input.lines() {
        let split_arr: Vec<&str> = line.split(&['=', ',', ':']).collect();

        let x1 = split_arr[1].parse::<i32>().unwrap();
        let y1 = split_arr[3].parse::<i32>().unwrap();
        let x2 = split_arr[5].parse::<i32>().unwrap();
        let y2 = split_arr[7].parse::<i32>().unwrap();

        let p1 = Point { x: x1, y: y1 };
        let p2 = Point { x: x2, y: y2 };

        let dist = p1.get_distance(&p2);
        sensors.push(p1);
        distances.push(dist);
    }

    // println!("sensors: {:?}", &sensors);
    // println!("distances: {:?}", &distances);
    // distances: [7, 1, 3, 4, 4, 5, 9, 10, 3, 8, 6, 5, 1, 7]

    // with upper_bounds = 4m
    // will take ~5_555 hours

    // for i in 0..upper_bounds {
    //     if i % 10_000 == 0 {
    //         println!("{}", i);
    //     }
    //     'j_loop: for j in 0..upper_bounds {
    //         let p = Point {
    //             x: i as i32,
    //             y: j as i32,
    //         };
    //         for i in 0..sensors.len() {
    //             let dist = sensors[i].get_distance(&p);
    //             if dist <= distances[i] {
    //                 // failed
    //                 // println!("within range of sensor {}", i);
    //                 continue 'j_loop;
    //             }
    //         }
    //         println!("======= point: {:?}", p);
    //         if p.x > 1000 {
    //             return p.x as usize;
    //         }
    //         return p.tuning_freq() as usize;
    //     }
    // }

    // how do I do it without looping a bazillion times?
    // each of the 26 beacons knows where it isn't
    // intersect each of them?
    // bounding box 4m x 4m
    // subtract each diamond
    // the resulting point is the answer?

    // check the edge of each sensor region

    for i in 0..sensors.len() {
        let point = &sensors[i];
        let distance = distances[i];
        let diamond = get_diamond_ring(point, distance + 1, upper_bounds as i32);

        'diamond: for p in diamond {
            // println!("Checking edge point: {:?}", &p);
            for j in 0..sensors.len() {
                if i == j {
                    continue;
                }
                let dist = sensors[j].get_distance(&p);

                if dist <= distances[j] {
                    // failed
                    // println!("within range of sensor {}", i);
                    continue 'diamond;
                }
            }
            println!("======= point: {:?}", p);
            // if p.x > 1000 {
            //     return p.x as usize;
            // }
            return p.tuning_freq() as usize;
        }
    }

    99
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

pub fn get_diamond_ring(center: &Point, distance: u32, upper_bounds: i32) -> Vec<Point> {
    let mut v = Vec::new();

    let x = center.x;
    let y = center.y;

    // dist 0 is self
    if distance == 0 {
        // v.push(self.get_coords(center));
        return v;
    }

    // need to keep negative offsets
    // for larger rings, maybe the edge is off the grid, but not all values are
    // so as it iterates, some might come back to non-negative

    // up
    let mut up_x = x as i32;
    let mut up_y = y + distance as i32;
    for _ in 0..distance {
        if up_x >= 0 && up_y >= 0 {
            if 0 <= up_x && up_x < upper_bounds && 0 <= up_y && up_y < upper_bounds {
                v.push(Point { x: up_x, y: up_y });
            }
            // v.push(self.get_xy(up_x as usize, up_y as usize));
        }
        // move down right
        up_x += 1;
        up_y -= 1;
    }
    // right
    let mut right_x = x + distance as i32;
    let mut right_y = y as i32;
    for _ in 0..distance {
        if right_x >= 0 && right_y >= 0 {
            if 0 <= right_x && right_x < upper_bounds && 0 <= right_y && right_y < upper_bounds {
                v.push(Point {
                    x: right_x,
                    y: right_y,
                });
            }

            // v.push(self.get_xy(right_x as usize, right_y as usize));
        }
        // move down left
        right_x -= 1;
        right_y -= 1;
    }
    // down
    let mut down_x = x as i32;
    let mut down_y = y as i32 - distance as i32;
    for _ in 0..distance {
        if down_x >= 0 && down_y >= 0 {
            if 0 <= down_x && down_x < upper_bounds && 0 <= down_y && down_y < upper_bounds {
                v.push(Point {
                    x: down_x,
                    y: down_y,
                });
            }

            // v.push(self.get_xy(down_x as usize, down_y as usize));
        }
        // move up left
        down_x -= 1;
        down_y += 1;
    }
    // left
    let mut left_x = x as i32 - distance as i32;
    let mut left_y = y as i32;
    for _ in 0..distance {
        if left_x >= 0 && left_y >= 0 {
            if 0 <= left_x && left_x < upper_bounds && 0 <= left_y && left_y < upper_bounds {
                v.push(Point {
                    x: left_x,
                    y: left_y,
                });
            }
            // v.push(self.get_xy(left_x as usize, left_y as usize));
        }
        // move up right
        left_x += 1;
        left_y += 1;
    }

    v
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn get_distance(&self, other: &Self) -> u32 {
        let x_diff = self.x.abs_diff(other.x);
        let y_diff = self.y.abs_diff(other.y);

        x_diff + y_diff
    }

    fn tuning_freq(&self) -> u64 {
        self.x as u64 * 4000000 + self.y as u64
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
    fn part_2_works() {
        assert_eq!(56000011, get_distress_frequency(&BASIC_INPUT_DAY_15, 20));
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

    #[test]
    fn test_tuning_freq() {
        let p = Point { x: 14, y: 11 };
        assert_eq!(56000011, p.tuning_freq());

        // let big: u64 = 4_000_000 * 4_000_000;
        // println!("big: {}", big);

        let big_p = Point {
            x: 4_000_000,
            y: 4_000_000,
        };
        assert_eq!(16000004000000, big_p.tuning_freq());
    }

    #[test]
    fn hashset_intersection() {
        let mut h1: HashSet<i32> = HashSet::new();
        h1.insert(1);
        h1.insert(2);
        h1.insert(3);
        let h2: HashSet<i32> = HashSet::from([3, 4, 5]);
        let h3: HashSet<i32> = HashSet::from([3, 5]);

        let h4: HashSet<i32> = h3.intersection(&h2).map(|x| *x).collect();
        // assert_eq!(h4, [3].iter().collect());
        // for x in h4.intersection(&h3) {

        // }

        assert_eq!(Some(&3), h1.intersection(&h4).next());
    }

    #[test]
    fn test_diamond() {
        let p = Point { x: 100, y: 250 };

        assert_eq!(
            vec![
                Point { x: 100, y: 251 },
                Point { x: 101, y: 250 },
                Point { x: 100, y: 249 },
                Point { x: 99, y: 250 },
            ],
            get_diamond_ring(&p, 1, 500)
        );

        let p = Point { x: 100, y: 250 };
        assert_eq!(4, get_diamond_ring(&p, 1, 500).len());

        let p = Point { x: 100, y: 250 };
        assert_eq!(8, get_diamond_ring(&p, 2, 500).len());

        let p = Point { x: 100, y: 250 };
        assert_eq!(12, get_diamond_ring(&p, 3, 500).len());
    }
}
