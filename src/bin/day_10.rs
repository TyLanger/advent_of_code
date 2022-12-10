use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/day_10_input.txt").unwrap();

    println!("{}", part_1(&input));
    println!("{}", part_2(&input)); // PGHFGLUG
}

fn part_1(input: &str) -> i32 {
    let v = get_signal_strengths(input);
    v.iter().sum()
}

fn part_2(input: &str) -> String {
    let mut output = "".to_string();
    let mut cycle = 0;
    let mut count = 1;

    for line in input.lines() {
        let split = line.split_once(' ');
        if let Some((_command, arg)) = split {
            let num: i32 = arg.parse().unwrap();

            cycle += 1;
            let letter = get_character(cycle, count);
            output = format!("{}{}", output, letter);
            if cycle % 40 == 0 {
                output = format!("{}\n", output);
            }

            cycle += 1;
            let letter = get_character(cycle, count);
            output = format!("{}{}", output, letter);
            if cycle % 40 == 0 {
                output = format!("{}\n", output);
            }

            count += num;
        } else {
            // noop
            cycle += 1;
            let letter = get_character(cycle, count);
            output = format!("{}{}", output, letter);
            if cycle % 40 == 0 {
                output = format!("{}\n", output);
            }
        }
    }
    output.trim_end().to_string()
}

fn get_signal_strengths(input: &str) -> Vec<i32> {
    let mut v = vec![0; 6];

    let mut count: i32 = 1;
    let mut cycle = 0;
    let mut index = 0;

    for line in input.lines() {
        let split = line.split_once(' ');
        if let Some((_command, arg)) = split {
            let num: i32 = arg.parse().unwrap();

            cycle += 1;
            if is_magic_cycle(cycle) {
                v[index] = count * cycle;
                index += 1;
            }

            cycle += 1;
            if is_magic_cycle(cycle) {
                v[index] = count * cycle;
                index += 1;
            }

            count += num;
        } else {
            // noop
            cycle += 1;
            if is_magic_cycle(cycle) {
                v[index] = count * cycle;
                index += 1;
            }
        }
        if cycle > 220 {
            break;
        }
    }

    // dbg!(&v);
    v
}

fn is_magic_cycle(current: i32) -> bool {
    // only care about 20th, 60th, 100th, 140th, 180th, 220th
    // not every 20
    // which is 20, then +40, +40 ...
    current == 20
        || current == 60
        || current == 100
        || current == 140
        || current == 180
        || current == 220
}

fn get_character(cycle: i32, sprite_pos: i32) -> String {
    // cycle can be between 1 and 40?
    // 1:1, 40:40, 41:1, 80:40
    let new_cycle = (cycle - 1) % 40 + 1;
    let diff = new_cycle.abs_diff(sprite_pos + 1);

    if diff < 2 {
        "#".to_string()
    } else {
        ".".to_string()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_1_works() {
        assert_eq!(13140, part_1(&BASIC_INPUT_DAY_10));
    }

    #[test]
    fn signal_strength_at_cycle() {
        let v = get_signal_strengths(&BASIC_INPUT_DAY_10);

        assert_eq!(420, v[0]);
        assert_eq!(1140, v[1]);
        assert_eq!(1800, v[2]);
        assert_eq!(2940, v[3]);
        assert_eq!(2880, v[4]);
        assert_eq!(3960, v[5]);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
                .to_string(),
            part_2(&BASIC_INPUT_DAY_10)
        );
    }

    #[test]
    fn visual_test() {
        println!(
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
        println!();
        println!("{}", part_2(&BASIC_INPUT_DAY_10));
        assert!(true);
    }

    #[test]
    fn get_right_character() {
        let char = get_character(1, 1);
        assert_eq!("#".to_string(), char);

        let char = get_character(2, 1);
        assert_eq!("#".to_string(), char);

        let char = get_character(3, 1);
        assert_eq!("#".to_string(), char);

        let char = get_character(4, 1);
        assert_eq!(".".to_string(), char);

        let char = get_character(41, 1);
        assert_eq!("#".to_string(), char);

        let char = get_character(81, 1);
        assert_eq!("#".to_string(), char);
    }

    const BASIC_INPUT_DAY_10: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
}
