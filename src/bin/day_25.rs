use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/day_25_input.txt").unwrap();

    // Sum: 35951702021395
    // 2-21=1==============
    // is not right
    // this snafu maps to:
    // 35_952_758_789_063
    // not the sum

    // 2-21=02=1-121-2-11-0
    // was missing an abs
    // I had the abs on the negative side
    // but the positive side turns them negative so
    // it's the side that needs abs
    println!("{}", part_1(&input));
}

fn part_1(input: &str) -> String {
    // add up all the numbers
    // output as a snafu number

    // I could convert to decimal
    // add up
    // convert back to snafu

    // could I just add snafu numbers?
    // 1 + 2 = 3
    // 1 + 2 = 1=
    // adding 1 goes:
    // =, -, 0, 1, 2
    // then 1=

    // places 625s, 125s, 25s, 5s, 1s
    // snafu 12111 = dec 906
    // decimal = 1*625 + 2*125 + 1*25 + 1*5 + 1*1 = 906

    let mut sum = 0;
    for line in input.lines() {
        let num = snafu_to_dec(line);
        sum += num;
    }

    // biggest I think
    // 1-=1=2=2==2==112122-
    // 20 digits
    // do I need i64?
    // 19 digits
    // MAX: i64 = 9_223_372_036_854_775_807i64

    println!("Sum: {:?}", sum);
    dec_to_snafu(sum)
}

fn snafu_to_dec(input: &str) -> i64 {
    let split: Vec<&str> = input.split("").skip(1).collect();

    let mut sum = 0;
    let mut place = 1;
    for &s in split.iter().rev().skip(1) {
        match s {
            "2" => {
                sum += 2 * place;
            }
            "1" => sum += place,
            "0" => {}
            "-" => sum -= place,
            "=" => sum -= 2 * place,
            _ => {}
        }
        place *= 5;
    }

    sum
}

fn dec_to_snafu(num: i64) -> String {
    // find the biggest power of 5 that it fits?
    // while num > place {
    //     place *= 5;
    // }

    // 10 / 5 = 2r0 == 20
    // 9 / 5 = 1r4  == 2-
    // 8 / 5 = 1r3  == 2=
    // 7 / 5 = 1r2  == 12
    // 6 / 5 = 1r1  == 11
    // 5/5 = 1r0    == 10
    // 4/5 = 0r4    == 1-
    // 3/5 = 0r3    == 1=
    // 2/5 = 0r2    == 2
    // 1/5 = 0r1    == 1

    // r0 = 0
    // r4 = -
    // r3 = =
    // r2 = 2
    // r1 = 1

    // 11 == 21
    // 11 / 5 = 2r1
    // 12 == 22
    // 12 / 5 = 2r2
    // 13 == 1==
    // 13 / 5 = 2r3
    // 13 / 25 = 0r13

    // 2 digits starts at 3
    // 2 + 1
    // 3 digits starts at 13
    // 10 + 2 + 1
    // 4 digits starts at 63
    // 50+10+2+1, 1===
    // 125 - 50 - 10 - 2 = 63
    // 50 + 10 + 2 = 62
    // 250 + 50 + 10 + 2 = 187
    // 312 is the biggest 4-digit, 63 the smallest

    // 63
    // find the digits = 4
    // find the big number = 125
    // num - big = -62
    // find the next biggest number = 50
    // if negative, add the number
    // -62 + 50
    // still neg, add 10
    // -12 + 10
    // -2 + 2

    // 312
    // 312 - 250 = 62
    // 62 - 50 = 12
    // 12 - 10 = 2
    // 2 - 2 = 0

    let digits = get_digits(num);
    let powers = get_powers(digits);
    let mut snafu = Vec::new();
    let mut smaller = num;
    for i in 0..digits {
        let place: i64 = 5_i64.pow(((digits - 1) - i) as u32);
        // let place = powers[digits-1-i];
        // 1, 5, 25, 125, 625
        // println!("Place: {}. Smaller: {}", place, smaller);

        if smaller == 0 {
            snafu.push("0");
            continue;
        }

        // 3 == 1=
        // 5 - 3 = 2

        // 6 == 11
        // place = 5
        // 2*5 == 10

        // 4890 == 2=-1=0
        // 6250 -1250 -125 +25 -10 +0
        // got 2=-2==
        // giving a 2 when it should give 1

        // it's 2 when it's bigger than 1222222
        // it's 1 when that many digits and
        // less than or equal to 122222

        // 15547668
        //      2=0-00112-=
        // got: 2=0-1======

        let p_max = powers_max(&powers, (digits - i) as usize);
        print!("Smaller: {} power_max: {}", smaller, p_max);
        if smaller.abs() > p_max {
            println!(" >");
        } else {
            println!(" <");
        }
        // Sum: 4890
        // Smaller: 4890 power_max: 4687
        // Smaller: -1360 power_max: 937
        // Smaller: -110 power_max: 187
        // Smaller: 15 power_max: 37
        // Smaller: -10 power_max: 7
        // 0 == 0
        // 2=-1=0
        //      a > b,      2
        // neg, a > b,      =
        // neg, a < b,      -
        //      a < b,      1
        // neg, a > b,      =

        // haven't tested internal 0s
        // when do I want 0s?
        // when the difference would be bigger than powers_max?
        // if adding or subbing would make a bigger number?
        // 100 sub 900 = -800
        // 800 > 100

        // is the issue 00?

        if smaller > 0 {
            let dig = (digits - i) as usize;

            if dig == 0 {
                match smaller {
                    2 => snafu.push("2"),
                    1 => snafu.push("1"),
                    _ => {}
                }
                continue;
            }

            if smaller > p_max {
                smaller = smaller - (2 * place);
                // println!("Sub 2 place: {}", 2*place);
                snafu.push("2");
            } else if (smaller - place).abs() < smaller {
                // missed this abs
                // this is where subtracting happens
                // so this needs the abs, not the other branch
                smaller = smaller - place;
                // println!("Sub 1 place: {}", place);

                snafu.push("1");
            } else {
                snafu.push("0");
            }
        } else {
            if smaller.abs() > p_max {
                smaller = smaller + (2 * place);
                // println!("Add 2 place: {}", 2*place);

                snafu.push("=");
            } else if (smaller + place).abs() < smaller.abs() {
                smaller = smaller + place;
                // println!("Add 1 place: {}", place);

                snafu.push("-");
            } else {
                snafu.push("0");
            }
        }
    }

    // println!("snafu: {:?}", snafu);

    let mut output = "".to_string();
    for s in snafu {
        output = output + s;
    }

    output
}

fn get_digits(num: i64) -> i64 {
    let mut place = 1;
    let mut digits = 1;
    let mut sum = 2;
    while num > sum {
        digits += 1;
        place *= 5;
        sum += 2 * place;
    }

    // println!("place: {}. Sum: {}", place, sum);

    digits
}

fn get_powers(digits: i64) -> Vec<i64> {
    let mut v = Vec::new();
    let mut place = 1;
    for _i in 0..digits {
        v.push(place);
        place *= 5;
    }
    v
}

fn powers_max(powers: &Vec<i64>, offset: usize) -> i64 {
    let mut sum = 0;

    for i in 0..offset {
        sum += 2 * powers[i];
    }
    // last only does half
    // max is 122222
    // not 222222
    sum -= powers[offset - 1];

    sum
}

#[cfg(test)]
mod tests {

    use super::*;

    const BASIC_INPUT_DAY_25: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn part_1_works() {
        assert_eq!("2=-1=0".to_string(), part_1(&BASIC_INPUT_DAY_25));
    }

    #[test]
    fn snafu_to_dec_test() {
        let snafu = "1";
        assert_eq!(1, snafu_to_dec(snafu));

        let snafu = "2";
        assert_eq!(2, snafu_to_dec(snafu));

        assert_eq!(3, snafu_to_dec("1="));
        assert_eq!(4, snafu_to_dec("1-"));
        assert_eq!(5, snafu_to_dec("10"));
        assert_eq!(6, snafu_to_dec("11"));
        assert_eq!(7, snafu_to_dec("12"));
        assert_eq!(8, snafu_to_dec("2="));
        assert_eq!(10, snafu_to_dec("20"));
        assert_eq!(11, snafu_to_dec("21"));
        assert_eq!(12, snafu_to_dec("22"));
        assert_eq!(13, snafu_to_dec("1=="));
        assert_eq!(14, snafu_to_dec("1=-"));
        assert_eq!(15, snafu_to_dec("1=0"));
        assert_eq!(50 + 10 + 2, snafu_to_dec("222"));
        assert_eq!(50 + 10 + 2 + 1, snafu_to_dec("1==="));
        assert_eq!(15547668, snafu_to_dec("2=0-00112-="));
        // sum: 35_951_702_021_395
        assert_eq!(35_952_758_789_063, snafu_to_dec("2-21=1=============="));
    }

    #[test]
    fn dec_to_snafu_test() {
        let num = 1;
        assert_eq!("1", dec_to_snafu(num));

        assert_eq!("2", dec_to_snafu(2));
        assert_eq!("1=", dec_to_snafu(3));
        assert_eq!("1-", dec_to_snafu(4));
        assert_eq!("10", dec_to_snafu(5));
        assert_eq!("11", dec_to_snafu(6));
        assert_eq!("12", dec_to_snafu(7));
        assert_eq!("22", dec_to_snafu(12));
        assert_eq!("1==", dec_to_snafu(13));
        assert_eq!("1===", dec_to_snafu(63));
        assert_eq!("1==-", dec_to_snafu(64));
        assert_eq!("1==0", dec_to_snafu(65));
        assert_eq!("1==1", dec_to_snafu(66));
        assert_eq!("1==2", dec_to_snafu(67));
        assert_eq!("1=-=", dec_to_snafu(68));
        assert_eq!("1-0---0", dec_to_snafu(12345));
        assert_eq!("2=0-00112-=", dec_to_snafu(15547668));
        assert_eq!("1121-1110-1=0", dec_to_snafu(314159265));
        assert_eq!("2=-1=0", dec_to_snafu(4890));
        // 6250 -1250 -125 +25 -10 +0
    }

    #[test]
    fn test_num_digits() {
        assert_eq!(1, get_digits(1));
        assert_eq!(1, get_digits(2));
        assert_eq!(2, get_digits(3));
        assert_eq!(2, get_digits(4));
        assert_eq!(2, get_digits(5));
        assert_eq!(3, get_digits(13));
        assert_eq!(4, get_digits(63));
    }

    #[test]
    fn test_powers() {
        assert_eq!(vec![1], get_powers(1));
        assert_eq!(vec![1, 5], get_powers(2));
        assert_eq!(vec![1, 5, 25], get_powers(3));
        assert_eq!(vec![1, 5, 25, 125, 625, 3125, 15625], get_powers(7));
    }

    #[test]
    fn test_power_sum() {
        let powers = get_powers(5);
        assert_eq!(1, powers_max(&powers, 1));
        assert_eq!(7, powers_max(&powers, 2));
        assert_eq!(37, powers_max(&powers, 3));
    }
}
