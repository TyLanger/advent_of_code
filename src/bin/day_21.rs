use std::{
    collections::{HashMap, VecDeque},
    fs,
};

fn main() {
    let input = fs::read_to_string("./inputs/day_21_input.txt").unwrap();

    println!("{}", part_1(&input));
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



    99
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
    fn part_1_works() {
        assert_eq!(152, part_1(&BASIC_INPUT_21_NUM));
    }

    #[test]
    fn part_2_works() {
        assert_eq!(301, part_2(&BASIC_INPUT_21_NUM));
    }
}
