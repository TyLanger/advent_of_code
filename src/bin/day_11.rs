use std::{collections::VecDeque, fs};

fn main() {
    let input = fs::read_to_string("./inputs/day_11_input.txt").unwrap();

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    get_monkey_business(input, 3, 20)
}

fn part_2(input: &str) -> usize {
    get_monkey_business(input, 1, 10_000)
}

fn get_monkey_business(input: &str, worry_reducer: usize, rounds: u32) -> usize {
    // queue of items
    // pop
    // mul by x
    // div by 3
    // check %
    // toss to another monkey

    // keep track of num items each monkey inspects
    // get top 2 monkeys
    // mul their inspects together

    // 20 rounds

    let input_splits: Vec<&str> = input.split("Monkey").collect();
    let mut v_monkeys = Vec::new();

    // to stop numbers from getting too big
    // number with factors made from each 'test' on each monkey
    let mut reducer = 1;

    for monkey_input in input_splits.iter().skip(1) {
        let monkey = Monkey::new(monkey_input, worry_reducer);
        reducer *= monkey.test;

        v_monkeys.push(monkey);
    }

    // dbg!(reducer);
    // 96577
    // reducer * reducer will overflow if u32

    let mut v_count = vec![0; v_monkeys.len()];

    // for each monkey in the list,
    // pop each item
    // get new value
    // push onto new monkey

    for _round in 0..rounds {
        for i in 0..v_monkeys.len() {
            while let Some(new_value_monkey) = v_monkeys[i].get_new_value_new_monkey() {
                v_count[i] += 1;
                let reduced_value = new_value_monkey.0 % reducer;
                v_monkeys[new_value_monkey.1].items.push_back(reduced_value);
            }
        }
    }

    v_count.sort_by(|a, b| b.cmp(a));

    v_count[0] * v_count[1]
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Operator,
    worry_reduction: usize,
    test: usize,
    true_monkey_index: usize,
    false_monkey_index: usize,
}

impl Monkey {
    fn new(input: &str, worry_reduction: usize) -> Self {
        // dbg!(input);
        let mut lines = input.lines();
        let _num = lines.next();

        let starting = lines.next();
        let op = lines.next();
        let test_str = lines.next();
        let true_index = lines.next();
        let false_index = lines.next();

        let mut items: VecDeque<usize> = VecDeque::new();
        let item_num_str: Vec<&str> = starting
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .split(',')
            .collect();
        for item in item_num_str {
            items.push_back(item.trim().parse().unwrap());
        }

        // contains
        // _new = old * 19
        let operation_str = op.unwrap().split_once(':').unwrap().1;
        let operation = Operator::new(operation_str);

        let test = test_str
            .unwrap()
            .split_once("by ")
            .unwrap()
            .1
            .trim()
            .parse()
            .unwrap();

        let true_monkey_index = true_index
            .unwrap()
            .split_once("monkey ")
            .unwrap()
            .1
            .trim()
            .parse()
            .unwrap();
        let false_monkey_index = false_index
            .unwrap()
            .split_once("monkey ")
            .unwrap()
            .1
            .trim()
            .parse()
            .unwrap();

        Monkey {
            items,
            operation,
            worry_reduction,
            test,
            true_monkey_index,
            false_monkey_index,
        }
    }

    fn get_new_value(&mut self) -> Option<usize> {
        let first = self.items.pop_front();
        if let Some(old_value) = first {
            Some(self.operation.calculate(old_value))
        } else {
            None
        }
    }

    fn get_monkey_throw_index(&self, value: usize) -> usize {
        if value % self.test == 0 {
            self.true_monkey_index
        } else {
            self.false_monkey_index
        }
    }

    fn get_new_value_new_monkey(&mut self) -> Option<(usize, usize)> {
        let value = self.get_new_value()?;
        let update_value = value / self.worry_reduction;
        let monkey = self.get_monkey_throw_index(update_value);
        Some((update_value, monkey))
    }
}

#[derive(PartialEq, Debug)]
struct Operator {
    first_value: OpValue,
    operation: MathSymbol,
    second_value: OpValue,
}

impl Operator {
    fn new(input: &str) -> Self {
        // contains
        // _new = old * 19
        let right = input.split_once("= ").unwrap().1;
        // v[old, *, 19]
        let values: Vec<&str> = right.split(' ').collect();

        let first_value = if values[0] == "old" {
            OpValue::Old
        } else {
            let val = values[0].parse().unwrap();
            OpValue::Value(val)
        };

        let operation = if values[1] == "*" {
            MathSymbol::Mul
        } else {
            MathSymbol::Plus
        };

        let second_value = if values[2] == "old" {
            OpValue::Old
        } else {
            let val = values[2].parse().unwrap();
            OpValue::Value(val)
        };

        Operator {
            first_value,
            operation,
            second_value,
        }
    }

    fn calculate(&self, value: usize) -> usize {
        let first = match self.first_value {
            OpValue::Old => value,
            OpValue::Value(x) => x,
        };
        let second = match self.second_value {
            OpValue::Old => value,
            OpValue::Value(x) => x,
        };
        match self.operation {
            MathSymbol::Plus => first + second,
            MathSymbol::Mul => first * second,
        }
    }
}

#[derive(PartialEq, Debug)]

enum OpValue {
    Old,
    Value(usize),
}

#[derive(PartialEq, Debug)]

enum MathSymbol {
    Plus,
    Mul,
}

#[cfg(test)]
mod tests {

    use super::*;

    const BASIC_INPUT_DAY_11: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    // #[ignore = "not ready"]
    fn part_1_works() {
        assert_eq!(10605, part_1(&BASIC_INPUT_DAY_11));
    }

    #[test]
    // #[ignore = "not ready"]
    fn part_2_works() {
        assert_eq!(2713310158, part_2(&BASIC_INPUT_DAY_11));
    }

    #[test]
    fn make_1_monkey() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3";

        let m = Monkey::new(input, 3);
        let mut expected = VecDeque::new();
        expected.push_back(79);
        expected.push_back(98);

        assert_eq!(expected, m.items);

        let op = Operator {
            first_value: OpValue::Old,
            operation: MathSymbol::Mul,
            second_value: OpValue::Value(19),
        };
        assert_eq!(op, m.operation);

        assert_eq!(23, m.test);
        assert_eq!(2, m.true_monkey_index);
        assert_eq!(3, m.false_monkey_index);

        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = 12 + old
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3";

        let m = Monkey::new(input, 3);

        let op = Operator {
            first_value: OpValue::Value(12),
            operation: MathSymbol::Plus,
            second_value: OpValue::Old,
        };
        assert_eq!(op, m.operation);
    }

    #[test]
    fn get_new_value_of_item() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3";

        let mut m = Monkey::new(input, 3);

        let value = m.get_new_value();

        assert_eq!(Some(79 * 19), value);
    }

    #[test]
    fn get_new_value_new_monkey() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3";

        let mut m = Monkey::new(input, 3);

        let value = m.get_new_value_new_monkey();

        assert_eq!(500, value.unwrap().0);
        assert_eq!(3, value.unwrap().1);

        let value = m.get_new_value_new_monkey();

        assert_eq!(620, value.unwrap().0);
        assert_eq!(3, value.unwrap().1);

        let value = m.get_new_value_new_monkey();

        assert_eq!(None, value);
    }

    #[test]
    fn changeable_worry() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3";

        let mut m = Monkey::new(input, 1);

        let value = m.get_new_value_new_monkey();

        // div by 1 (part 2)
        assert_eq!(1501, value.unwrap().0);
        assert_eq!(3, value.unwrap().1);

        let value = m.get_new_value_new_monkey();

        assert_eq!(1862, value.unwrap().0);
        assert_eq!(3, value.unwrap().1);

        let value = m.get_new_value_new_monkey();

        assert_eq!(None, value);
    }

    #[test]
    fn smaller_numbers() {
        let reducer: u32 = 23 * 19 * 13 * 17;

        let big_number = 100_100_100;
        let small_number = big_number % reducer;

        assert!(small_number < big_number);
    }
}
