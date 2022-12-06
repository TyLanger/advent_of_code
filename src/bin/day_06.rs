use std::fs;

fn main() {
    // remember to change the input
    let file = fs::read_to_string("./inputs/day_06_input.txt").unwrap();
    println!("{}", part_1(&file));
    println!("{}", part_2(&file));
}

fn part_1(input: &str) -> usize {
    
    // are the last 4 letters different?

    let mut a = "";
    let mut b = "";
    let mut c = "";
    let mut d = "";

    for (i, letter) in input.split("").enumerate().skip(1) {

        if i % 4 == 0 {
            a = letter;
        } else if i % 4 == 1 {
            b = letter;
        } else if i % 4 == 2 {
            c = letter;
        } else {
            d = letter;
        }

        if i >= 4 {

        
        if all_different(a, b, c, d) {
            return i;
        }
    }
    }
    return 99;
}

fn part_2(input: &str) -> usize {
    let mut v = vec![""; 14];

    for (i, letter) in input.split("").enumerate().skip(1) {

        let remainder = i % 14;

        v[remainder] = letter;

        if all_different_vec(&v) {
            return i;
        }

    }

    99
}

fn all_different(a: &str, b: &str, c: &str, d: &str) -> bool {
    if a == b || a == c || a == d {
        return false;
    } else if b == c || b == d {
        return false;
    } else if c == d || d == "" {
        return false;
    } else {
        return true;
    }
}

fn all_different_vec(v: &Vec<&str>) -> bool {
    for i in 0..(v.len()-1) {
        for j in (i+1)..v.len() {
            if v[i] == v[j] {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT_1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const TEST_INPUT_2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const TEST_INPUT_3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const TEST_INPUT_4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const TEST_INPUT_5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn part_1_works() {
        let result = part_1(&TEST_INPUT_1);
        assert_eq!(7, result);

        let result = part_1(&TEST_INPUT_2);
        assert_eq!(5, result);

        let result = part_1(&TEST_INPUT_3);
        assert_eq!(6, result);

        let result = part_1(&TEST_INPUT_4);
        assert_eq!(10, result);

        let result = part_1(&TEST_INPUT_5);
        assert_eq!(11, result);
    }

    #[test]
    fn part_2_works() {
        let result = part_2(&TEST_INPUT_1);
        assert_eq!(19, result);

        let result = part_2(&TEST_INPUT_2);
        assert_eq!(23, result);

        let result = part_2(&TEST_INPUT_3);
        assert_eq!(23, result);

        let result = part_2(&TEST_INPUT_4);
        assert_eq!(29, result);

        let result = part_2(&TEST_INPUT_5);
        assert_eq!(26, result);
    }

}