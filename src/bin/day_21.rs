use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

fn main() {
    let input = fs::read_to_string("./inputs/day_21_input.txt").unwrap();

    // println!("{}", part_1(&input));
    // println!("{}", part_2(&input));
    println!("{}", part_2_guess(&input));
}

fn part_1(input: &str) -> i64 {
    // put each monkey in a hashmap
    // go from root and expand out until I have an answer
    // recursive / tree structure
    // the left and right might both depend on the same other monkey
    //      A
    //     B  C
    //    D E D F
    // I would calculate D twice

    // I could go through all the monkeys
    // and reduce them down to numbers
    // might need to loop 1500 times if there is a long chain
    // of dependencies

    // keep track of each monkey with a number
    // iterate through the operations
    // if I can simplify it, do and add it to the number set

    let mut monkey_map: HashMap<String, Monkey> = HashMap::new();
    let mut values = HashMap::new();

    for line in input.lines() {
        let once = line.split_once(':').unwrap();
        let name = once.0.to_string();
        let monkey_yell: Vec<&str> = once.1.split(' ').skip(1).collect();

        if monkey_yell.len() > 1 {
            let op = match monkey_yell[1] {
                "+" => Operator::Add,
                "-" => Operator::Subtract,
                "*" => Operator::Multiply,
                "/" => Operator::Divide,
                x => panic!("didn't match +-*/: {}", x),
            };
            let operation = Operation {
                left: monkey_yell[0].trim().to_string(),
                op,
                right: monkey_yell[2].trim().to_string(),
            };
            let monkey = Monkey {
                name: name.clone(),
                job: MonkeyJob::Op(operation),
            };
            // println!("{:?}: {:?}", &name, &monkey);
            monkey_map.insert(name, monkey);
        } else {
            let value = monkey_yell[0].trim().parse::<i64>().unwrap();

            values.insert(name, value);
        }
    }

    // let mut to_evaluate = HashSet::new();
    let mut queue = VecDeque::new();

    let root = monkey_map.get(&"root".to_string()).unwrap();
    queue.push_back(root);

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();

        if queue.is_empty() && current.name != "root" {
            if !values.contains_key("root") {
                // empty options first
                // i.e. go to the depth of the tree first
                // println!("Emptied");
                queue.push_back(root);
            }
        }

        if let MonkeyJob::Op(o) = &current.job {
            // are both ops already numbers?
            // println!("Try {:?}", current.name);

            let left = values.get(&o.left);
            let right = values.get(&o.right);

            if let (Some(left_val), Some(right_val)) = (left, right) {
                match o.op {
                    Operator::Add => {
                        values.insert(current.name.clone(), left_val + right_val);
                    }
                    Operator::Subtract => {
                        values.insert(current.name.clone(), left_val - right_val);
                    }
                    Operator::Multiply => {
                        values.insert(current.name.clone(), left_val * right_val);
                    }
                    Operator::Divide => {
                        values.insert(current.name.clone(), left_val / right_val);
                    }
                }
                continue;
            }

            if left.is_none() {
                queue.push_back(monkey_map.get(&o.left).unwrap());
            }
            if right.is_none() {
                queue.push_back(monkey_map.get(&o.right).unwrap());
            }
            // I think this is slower
            // it's very optimistic
            // it checks root very often
            // queue.push_back(current); // retry?
        } else {
            // don't think I need this
            if let MonkeyJob::Value(v) = &current.job {
                values.insert(current.name.clone(), *v);
            }
        }
    }

    if let MonkeyJob::Op(op) = &root.job {
        if let Some(&left) = values.get(&op.left) {
            println!("left side: {:?}", left);
        }
        println!("{:?}", op.op);
        if let Some(&right) = values.get(&op.right) {
            println!("right side: {:?}", right);
        }
    }

    // println!("to eval: {:?} len: {:?}", to_evaluate, to_evaluate.len());
    // println!("values: {:?} len: {:?}", values, values.len());

    *values.get(&"root".to_string()).unwrap()
}

fn part_2(input: &str) -> i64 {
    // you are humn
    // what number do you need to shout out
    // to then propagate up so that both sides of root monkey match

    // figure out which side of monkey uses humn?
    // maybe both will?

    // try to evaluate all monkeys
    // will end up with
    // root: aa == bb
    // aa = cc + dd
    // bb = ee + ff
    // cc = humn + gg
    // dd = 7
    // ee = 12
    // ff = 9
    // gg = 1
    // mix of numbers and operations

    // humn = 4905
    // gvhn: humn - zbzz
    // only 1 thing uses humn
    // so I shouldn't have to worry about it being on both sides
    // can I evaluate the one side of root?
    // and get a concrete number and then work backwards?
    // do opposite operations?
    // + becomes -, * becomes /?
    //      root
    //    A   =   B
    //  3 * 4    17 - humn
    // 12 = 17 - humn
    // 12 + 17 = 29. 71 - 12 = 5
    // humn = 5\

    // left = B = C - humn
    // left = c - humn
    // left - c = -humn

    // version 2
    // guess a number
    // was it right?
    // was it too high or too low?
    // if it was too high, guess a lower number
    // keep track of guesses and refine the search until it is right

    // do part 1, but ignore humn
    // see what we get
    // how do I get it to end?

    // v3
    // create the equation then solve for humn
    // how?
    // 23472 = ((a+b) / (c-d)) * (humn - 4)
    // 150 = (6 + humn) => 150 -6 = humn
    // 150 = humn + 6 => 150 - 6 = humn same
    // 150 = 6 - humn => 150 + humn = 6 => humn = 6 - 150
    // 150 = humn - 6 => 150 + 6 = humn
    // 150 = humn * 6 => 150 / 6 = humn
    // 150 = humn / 6 => 150 * 6 = humn
    // 150 = 6 / humn => 150 * humn = 6 => humn = 6 / 150

    // v4
    // tree
    // shift the nodes around
    // rebalance and stuff

    // v5
    // simplify
    // have a enum cant, maybe
    // if humn, cant
    // if one of your leaves is cant, you are also cant
    // keep going until the root is cant
    // then I have everything in simplest form
    // the left side is a number
    // the right side is one deep branch and the rest are numbers probs

    let mut monkey_map: HashMap<String, Monkey> = HashMap::new();
    let mut values = HashMap::new();

    for line in input.lines() {
        let once = line.split_once(':').unwrap();
        let name = once.0.to_string();
        let monkey_yell: Vec<&str> = once.1.split(' ').skip(1).collect();

        if monkey_yell.len() > 1 {
            let op = match monkey_yell[1] {
                "+" => Operator::Add,
                "-" => Operator::Subtract,
                "*" => Operator::Multiply,
                "/" => Operator::Divide,
                x => panic!("didn't match +-*/: {}", x),
            };
            let operation = Operation {
                left: monkey_yell[0].trim().to_string(),
                op,
                right: monkey_yell[2].trim().to_string(),
            };
            let monkey = Monkey {
                name: name.clone(),
                job: MonkeyJob::Op(operation),
            };
            // println!("{:?}: {:?}", &name, &monkey);
            monkey_map.insert(name, monkey);
        } else {
            let value = monkey_yell[0].trim().parse::<i64>().unwrap();

            if name != "humn" {
                values.insert(name, value);
            }
        }
    }

    let mut queue = VecDeque::new();

    let mut cant = HashSet::new();
    cant.insert("humn".to_string());

    let root = monkey_map.get(&"root".to_string()).unwrap();
    queue.push_back(root);

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();

        if cant.contains("root") {
            break;
        }

        if queue.is_empty() && current.name != "root" {
            if !values.contains_key("root") {
                // empty options first
                // i.e. go to the depth of the tree first
                // println!("Emptied");
                queue.push_back(root);
            }
        }

        if let MonkeyJob::Op(o) = &current.job {
            let left = values.get(&o.left);
            let right = values.get(&o.right);

            // are both ops already numbers?
            // println!("Try {:?}", current.name);
            let left_cant = cant.contains(&o.left);
            let right_cant = cant.contains(&o.right);
            if left_cant || right_cant {
                // add self to cant
                cant.insert(current.name.clone());
            } else {
                if let (Some(left_val), Some(right_val)) = (left, right) {
                    match o.op {
                        Operator::Add => {
                            values.insert(current.name.clone(), left_val + right_val);
                        }
                        Operator::Subtract => {
                            values.insert(current.name.clone(), left_val - right_val);
                        }
                        Operator::Multiply => {
                            values.insert(current.name.clone(), left_val * right_val);
                        }
                        Operator::Divide => {
                            values.insert(current.name.clone(), left_val / right_val);
                        }
                    }
                    continue;
                }
            }

            if left.is_none() && !left_cant {
                queue.push_back(monkey_map.get(&o.left).unwrap());
            }
            if right.is_none() && !right_cant {
                queue.push_back(monkey_map.get(&o.right).unwrap());
            }
            // I think this is slower
            // it's very optimistic
            // it checks root very often
            // queue.push_back(current); // retry?
        } else {
            // don't think I need this
            if let MonkeyJob::Value(v) = &current.job {
                values.insert(current.name.clone(), *v);
            }
        }
    }
    // println!("Values: {:?}", values);
    println!("Cant: {:?} len: {}", cant, cant.len());

    let mut root_eq = 0;
    if let MonkeyJob::Op(op) = &root.job {
        if let Some(&left) = values.get(&op.left) {
            println!("left side: {:?}", left);
            root_eq = left;
        }
        if let Some(&right) = values.get(&op.right) {
            println!("right side: {:?}", right);
            root_eq = right;
        }
    }

    // 150 = a+b+c/d*e-g+humn
    // for test have: root, a,b,c,d, humn

    // 7628196411405 = root + 68x + humn

    99
}

fn part_2_guess(input: &str) -> i64 {
    let mut monkey_map: HashMap<String, Monkey> = HashMap::new();
    let mut values = HashMap::new();

    for line in input.lines() {
        let once = line.split_once(':').unwrap();
        let name = once.0.to_string();
        let monkey_yell: Vec<&str> = once.1.split(' ').skip(1).collect();

        if monkey_yell.len() > 1 {
            let op = match monkey_yell[1] {
                "+" => Operator::Add,
                "-" => Operator::Subtract,
                "*" => Operator::Multiply,
                "/" => Operator::Divide,
                x => panic!("didn't match +-*/: {}", x),
            };
            let operation = Operation {
                left: monkey_yell[0].trim().to_string(),
                op,
                right: monkey_yell[2].trim().to_string(),
            };
            let monkey = Monkey {
                name: name.clone(),
                job: MonkeyJob::Op(operation),
            };
            // println!("{:?}: {:?}", &name, &monkey);
            monkey_map.insert(name, monkey);
        } else {
            let mut value = monkey_yell[0].trim().parse::<i64>().unwrap();

            if name == "humn" {
                // humn is on right
                // humn is on the left???
                // 1 left bigger
                // 1000 left bigger
                // 100_000 left bigger
                // 10m left bigger
                // 100m, left bigger
                // 4000, left bigger by 9_690_962_552_796
                // 4_000_000_000_000, right bigger by 307_690_635_852
                // 40_T, righ bigger by 90_295_569_423_732
                // 2 T, left bigger by 4_691_635_963_476
                // 3 T left bigger by 2_191_972_663_812
                // 4 T right bigger by 307_690_635_852
                // 5 T right bigger by 2_807_353_935_516
                // 3.5T left bigger by 942_141_013_980
                // 3.5T ..|. 4T
                // 3.75T
                // 3.625T left bigger by 629_683_101_516
                // 3.875 left bigger by     4_767_276_600
                // 3.9375 right bigger by 151_461_679_632
                // humn guess: 3_906_250_000_000
                // right bigger by 73_347_201_516
                // humn guess: 3890625000000
                // right bigger by 34_289_962_452
                // humn guess: 3882812500000
                // right bigger by 14_761_342_932
                // humn guess: 3878906250000
                // right bigger by 4_997_033_160
                // humn guess: 3876953125000
                // right bigger by 114_878_280
                // humn guess: 3875976562500
                // left bigger by 2_326_199_160
                // humn guess: 3876464843750
                // left bigger by 1_105_660_440
                // humn guess: 3876708984375
                // left bigger by 495_391_080
                // humn guess: 3876831054687
                // left bigger by 190_256_400
                // humn guess: 3876892089843
                // left bigger by 37_689_072
                // humn guess: 3876922607421
                // right bigger by 38_594_604
                // humn guess: 3876907348632
                // right bigger by 452_772
                // humn guess: 3876899719237
                // left bigger by 18_618_144
                // humn guess: 3876903533934
                // left bigger by 9_082_704
                // humn guess: 3876905441283
                // left bigger by 4_314_960
                // humn guess: 3876906394957
                // left bigger by 1_931_100
                // humn guess: 3876906871794
                // left bigger by 739_176
                // humn guess: 3876907110213
                // left bigger by 143_196
                // humn guess: 3876907229422
                // right bigger by 154_788
                // humn guess: 3876907169817
                // right bigger by 5_796
                // humn guess: 3876907140015
                // left bigger by 68_700
                // humn guess: 3876907154916
                // left bigger by 31_452
                // humn guess: 3876907167366
                // left bigger by 336
                // humn guess: 3876907167866
                // right bigger by 912
                // humn guess: 3876907167616
                // right bigger by 288
                // humn guess: 3876907167491
                // left bigger by 24
                // humn guess: 3876907167474
                // left bigger by 72
                // humn guess: 3876907167482
                // left bigger by 48
                // humn guess: 3876907167486
                // left bigger by 36
                // humn guess: 3876907167488
                // left bigger by 36
                // humn guess: 3876907167494
                // left bigger by 24

                // let upper = 3876907167499;
                // let lower = 3876907167497;
                // let middle = (upper + lower) / 2;

                // matches
                // 3876907167498 is too high
                // 3876907167497 is too high
                // 3876907167496
                // 3876907167495
                // nope 3876907167494

                value = 3876907167495;
                println!("humn guess: {:?}", value);
            }
            values.insert(name, value);
        }
    }

    // let mut to_evaluate = HashSet::new();
    let mut queue = VecDeque::new();

    let root = monkey_map.get(&"root".to_string()).unwrap();
    queue.push_back(root);

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();

        if queue.is_empty() && current.name != "root" {
            if !values.contains_key("root") {
                // empty options first
                // i.e. go to the depth of the tree first
                // println!("Emptied");
                queue.push_back(root);
            }
        }

        if let MonkeyJob::Op(o) = &current.job {
            // are both ops already numbers?
            // println!("Try {:?}", current.name);

            let left = values.get(&o.left);
            let right = values.get(&o.right);

            if let (Some(left_val), Some(right_val)) = (left, right) {
                match o.op {
                    Operator::Add => {
                        values.insert(current.name.clone(), left_val + right_val);
                    }
                    Operator::Subtract => {
                        values.insert(current.name.clone(), left_val - right_val);
                    }
                    Operator::Multiply => {
                        values.insert(current.name.clone(), left_val * right_val);
                    }
                    Operator::Divide => {
                        values.insert(current.name.clone(), left_val / right_val);
                    }
                }
                continue;
            }

            if left.is_none() {
                queue.push_back(monkey_map.get(&o.left).unwrap());
            }
            if right.is_none() {
                queue.push_back(monkey_map.get(&o.right).unwrap());
            }
            // I think this is slower
            // it's very optimistic
            // it checks root very often
            // queue.push_back(current); // retry?
        } else {
            // don't think I need this
            if let MonkeyJob::Value(v) = &current.job {
                values.insert(current.name.clone(), *v);
            }
        }
    }

    let mut left_value = 0;
    let mut right_value = 0;

    if let MonkeyJob::Op(op) = &root.job {
        if let Some(&left) = values.get(&op.left) {
            // println!("left side: {:?}", left);
            left_value = left;
        }
        // println!("{:?}", op.op);
        if let Some(&right) = values.get(&op.right) {
            // println!("right side: {:?}", right);
            right_value = right;
        }
        // println!("Match: {}", left_value == right_value);
        if left_value > right_value {
            println!("left bigger by {}", left_value - right_value);
        } else if left_value < right_value {
            println!("right bigger by {}", right_value - left_value);
        } else {
            println!("matches!");
        }
    }

    // the right side has humn

    *values.get(&"root".to_string()).unwrap()
}

#[derive(Debug)]
struct Monkey {
    name: String,
    job: MonkeyJob,
}

#[derive(Debug)]
enum MonkeyJob {
    Value(i64),
    Op(Operation),
}

#[derive(Debug)]
struct Operation {
    left: String,
    op: Operator,
    right: String,
}

#[derive(Debug)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[cfg(test)]
mod tests {

    use super::*;

    const BASIC_INPUT_21_NUM: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    #[ignore = "done"]
    fn part_1_works() {
        assert_eq!(152, part_1(&BASIC_INPUT_21_NUM));
    }

    #[test]
    fn part_2_works() {
        assert_eq!(301, part_2(&BASIC_INPUT_21_NUM));
    }
}
