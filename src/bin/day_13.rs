use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/day_13_input.txt").unwrap();

    // 4451 is too low
    println!("{}", part_1(&input));
}

fn part_1(input: &str) -> u32 {
    let mut lines = input.lines();
    let mut count = 0;
    let mut index = 0;
    loop {
        let a = lines.next();
        let b = lines.next();

        let packet_a = build_depth_list(a.unwrap());
        let packet_b = build_depth_list(b.unwrap());

        if index == 0 {
            println!("{:?}", packet_a);
            println!("{:?}", packet_b);
        }

        index += 1;

        let ordered = is_in_order(&packet_a, &packet_b);
        println!("index: {} was {}", index, ordered);
        if ordered {
            count += index;
        }

        let c = lines.next();
        if c.is_none() {
            break;
        }
    }

    count
}


#[allow(unused)]
fn in_right_order(v1: &[u32], v2: &[u32]) -> bool {
    for i in 0..v1.len() {
        if v1[i] < v2[i] {
            return true;
        }
    }

    false
}

fn is_in_order(v1: &[Depth], v2: &[Depth]) -> bool {
    let len = v1.len().min(v2.len());
    for i in 0..len {
        let a = &v1[i];
        let b = &v2[i];

        // am I forgetting the case
        // where 1 list runs out first?
        // can only test with depths differing by 1 once?

        if a.depth == b.depth && a.list == b.list {
            if a.value.is_none() && b.value.is_some() {
                return true;
            } else if a.value.is_none() && b.value.is_none() {
                // both none
                continue;
            } else if a.value.is_some() && b.value.is_none() {
                return false;
            }

            if a.value.unwrap() < b.value.unwrap() {
                return true;
            } else if a.value.unwrap() > b.value.unwrap() {
                return false;
            }
            // == continue
        } else if a.depth.abs_diff(b.depth) == 1 {
            // within 1 depth still works?
            if a.value.is_none() && b.value.is_some() {
                return true;
            } else if a.value.is_none() && b.value.is_none() {
                if a.depth < b.depth {
                    return true;
                } else {
                    return false;
                }
            } else if a.value.is_some() && b.value.is_none() {
                return false;
            }

            if a.value.unwrap() < b.value.unwrap() {
                return true;
            } else if a.value.unwrap() > b.value.unwrap() {
                return false;
            }
            // == continue
        } else {
            return false;
        }
    }
    if v1.len() < v2.len() {
        return true;
    }

    false
}

#[allow(unused)]
fn get_match_length(s1: &str, s2: &str) -> usize {
    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();
    for i in 0..chars1.len() {
        if chars1[i] != chars2[i] {
            return i;
        }
    }
    0
}

#[allow(unused)]
fn get_number_depths(s1: &str) -> Vec<u32> {
    let chars: Vec<char> = s1.chars().collect();
    let mut v = Vec::new();
    let mut current_depth = 0;
    for c in chars {
        if c == '[' {
            current_depth += 1;
        } else if c == ']' {
            current_depth -= 1;
        } else if c == ',' {
            // skip
        } else {
            // number
            v.push(current_depth);
        }
    }
    v
}

fn build_depth_list(input: &str) -> Vec<Depth> {
    // println!("{:?}", input);
    // handle 2-digit numbers like 10
    // [[ will leave a "" so filter that out
    let numbers: Vec<&str> = input
        .split(&['[', ',', ']'])
        .filter(|x| !x.is_empty())
        .collect();
    let mut number_index = 0;
    // println!("After split: len: {} {:?} ", numbers.len(), numbers);

    let chars: Vec<char> = input.chars().collect();
    let mut v = Vec::new();
    let mut current_depth = 0;
    let mut list_num = 0;
    let mut open_was_last = false;
    let mut num_was_last = false;
    for c in chars {
        if c == '[' {
            current_depth += 1;
            list_num += 1;
            open_was_last = true;
            num_was_last = false;
        } else if c == ']' {
            if open_was_last {
                let d = Depth {
                    value: None,
                    depth: current_depth,
                    list: list_num,
                };

                v.push(d);
            }
            current_depth -= 1;
            open_was_last = false;
            num_was_last = false;
        } else if c == ',' {
            // skip
            open_was_last = false;
            num_was_last = false;
        } else {
            // number
            // handle 2-digit cases
            if !num_was_last {
                let d = Depth {
                    value: Some(numbers[number_index].trim().parse::<u32>().unwrap()),
                    depth: current_depth,
                    list: list_num,
                };
                number_index += 1;

                v.push(d);
                open_was_last = false;
                num_was_last = true;
            }
        }
    }
    v
}

#[derive(Debug, Copy, Clone)]
struct Depth {
    value: Option<u32>,
    depth: u32,
    list: u32,
}

#[allow(unused)]
#[derive(Debug)]
enum Symbol {
    Start,
    End,
    Value(u32),
    Empty,
}

#[allow(unused)]
fn parse_input(input: &str) -> Vec<Symbol> {

    let numbers: Vec<&str> = input
        .split(&['[', ',', ']'])
        .filter(|x| !x.is_empty())
        .collect();
    let mut number_index = 0;

    let mut v = Vec::new();
    let split = input.split("").filter(|x| !x.is_empty());

    let mut empty = true;
    let mut number_last = false;
    for s in split {
        if s == "[" {
            v.push(Symbol::Start);
            empty = true;
            number_last = false;
        } else if s == "]" {
            if empty {
                v.push(Symbol::Empty);
            }
            v.push(Symbol::End);
            number_last = false;
        } else if s == "," {
            // skip
            number_last = false;
        } else {
            if !number_last {
                empty = false;
                v.push(Symbol::Value(numbers[number_index].trim().parse::<u32>().unwrap()));
                number_index += 1;
                number_last = true;
            }
        }
    }

    v
}

// fn recursion(chars: &Vec<char>, start: usize) -> Signal {
//     let v = Vec::new();
//     for i in start..chars.len() {
//         if chars[i] == '[' {
//             // Signal::List(())
//             // ??
//             let v1 = recursion(chars, i + 1);
//             v.push(v1);
//         } else if chars[i] == ']' {
//             return v;
//         } else if chars[i] == ',' {

//         } else {
//             // Signal::Number(chars[i])
//         }
//     }

//     v
// }

#[allow(unused)]
fn recurse_symbols(symbol_arr: &Vec<Symbol>, start_index: usize) -> Vec<Signal> {
    let mut v = Vec::new();
    
    let mut skip = 0;
    for i in start_index..symbol_arr.len() {
        // dbg!(&symbol_arr[i]);
        match symbol_arr[i+skip] {
            Symbol::Start => {
                println!("Start at {}", start_index);
                let vec = recurse_symbols(symbol_arr, i+1);
                println!("len {}", &vec.len());
                skip += &vec.len() + 2;

                let sig = Signal::List(vec);
                v.push(sig);
            },
            Symbol::End => {
                println!("End at {}", start_index);

                return v;
            },
            Symbol::Value(val) => {
                println!("Push {} at {}", val, start_index);

                v.push(Signal::Number(val));
            },
            Symbol::Empty => {
                println!("Empty at {}", start_index);

                v.push(Signal::Empty);
            },
        }
    }
    
    v
}

#[allow(unused)]
fn build_symbols_stack(symbol_arr: &Vec<Symbol>) -> Vec<Signal> {
    let mut v = Vec::new(); 

    let mut stack = Vec::new();

    let mut current = Vec::new();

    for i in 0..symbol_arr.len() {
        match symbol_arr[i] {
            Symbol::Start => {
                println!("Start");
                stack.push(current);
                let v1 = Vec::new();
                // stack.push(v1);
                current = v1;
            },
            Symbol::End => {
                println!("End");
                let popped = stack.pop().unwrap();
                if popped.len() > 0 {
                    v.push(Signal::List( popped));
                }
                // v.push(Signal::List(current));
            },
            Symbol::Value(val) => {
                println!("Push {}", val);

                let mut stack_top = stack.pop().unwrap();
                stack_top.push(Signal::Number(val));
                stack.push(stack_top);

                // current.push(Signal::Number(val));
            },
            Symbol::Empty => {
                println!("Empty");

                let mut stack_top = stack.pop().unwrap();
                stack_top.push(Signal::Empty);
                stack.push(stack_top);

                // current.push(Signal::Empty);
            },
        }
    }

    // for item in stack {
    //     v.push(Signal::List(item));
    // }
    
    v
}


#[derive(Debug)]
#[allow(unused)]
enum Signal {
    List(Vec<Signal>),
    Number(u32),
    Empty,
}

#[cfg(test)]
mod tests {

    use super::*;

    const BASIC_INPUT_DAY_13: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    // #[ignore = "not ready yet"]
    fn part_1_works() {
        assert_eq!(13, part_1(&BASIC_INPUT_DAY_13));
    }

    #[test]
    fn get_order_pair_1() {
        let v1 = vec![1, 1, 3, 1, 1];
        let v2 = vec![1, 1, 5, 1, 1];

        assert!(in_right_order(&v1, &v2));
    }

    #[test]
    fn construct_vec_of_vecs() {
        // rust doesn't like vecs like this
        // interior vecs need to all be the same length
        // let v1 = vec![[1],[2,3,4]];
        // let v2 = vec![[1],4];

        // assert!(in_right_order(&v1, &v2));

        // pair 2
        let mut v_of_v = Vec::new();
        let v1 = vec![1];
        let v2 = vec![2, 3, 4];
        v_of_v.push(v1);
        v_of_v.push(v2);

        println!("{:?}", v_of_v);

        let mut v_of_v2 = Vec::new();
        let v1 = vec![1];
        let v2 = vec![4];
        v_of_v2.push(v1);
        v_of_v2.push(v2);

        println!("{:?}", v_of_v2);

        // pair 3
        let mut pair_3_a = Vec::new();
        let v1 = vec![9];
        pair_3_a.push(v1);

        let mut pair_3_b = Vec::new();
        let v1 = vec![8, 7, 6];
        pair_3_b.push(v1);

        println!("{:?}", pair_3_a);
        println!("{:?}", pair_3_b);

        // pair 4
        // [[4,4],4,4]
        let mut pair_4_a = Vec::new();
        let v1 = vec![4, 4];
        pair_4_a.push(v1);
        // don't think I can represent this
        // pair_4_a.push(4);
    }

    #[test]
    fn get_match_length_test() {
        let s1 = "[1,1,3,1,1]";
        let s2 = "[1,1,5,1,1]";

        assert_eq!(5, get_match_length(s1, s2));

        let s1 = "[[1],[2,3,4]]";
        let s2 = "[[1],4]";

        assert_eq!(5, get_match_length(s1, s2));
    }

    #[test]
    fn create_signals() {
        // [1,1,3,1,1]
        let mut v = Vec::new();
        let a = Signal::Number(1);
        let b = Signal::Number(1);
        let c = Signal::Number(3);
        let d = Signal::Number(1);
        let e = Signal::Number(1);
        v.push(a);
        v.push(b);
        v.push(c);
        v.push(d);
        v.push(e);

        // [[1],[2,3,4]]
        let mut v = Vec::new();
        let a = Signal::List(vec![Signal::Number(1)]);
        let mut v2 = Vec::new();
        v2.push(Signal::Number(2));
        v2.push(Signal::Number(3));
        v2.push(Signal::Number(4));
        let b = Signal::List(v2);
        v.push(a);
        v.push(b);
    }

    #[test]
    fn recursion() {
        // each time I find a [, open a new list
        // when I reach a ], close the list (return)
    }

    #[test]
    fn get_list_depth() {
        let s1 = "[1,1,3,1,1]";
        let s2 = "[1,1,5,1,1]";
        // depth 1
        assert_eq!(vec![1, 1, 1, 1, 1], get_number_depths(s1));
        assert_eq!(vec![1, 1, 1, 1, 1], get_number_depths(s2));

        let s1 = "[[1],[2,3,4]]";
        let s2 = "[[1],4]";
        // depth 2
        assert_eq!(vec![2, 2, 2, 2], get_number_depths(s1));
        assert_eq!(vec![2, 1], get_number_depths(s2));

        let s1 = "[9]";
        let s2 = "[[8,7,6]]";
        assert_eq!(vec![1], get_number_depths(s1));
        assert_eq!(vec![2, 2, 2], get_number_depths(s2));

        let s1 = "[[4,4],4,4]";
        let s2 = "[[4,4],4,4,4]";
        assert_eq!(vec![2, 2, 1, 1], get_number_depths(s1));
        assert_eq!(vec![2, 2, 1, 1, 1], get_number_depths(s2));

        let s1 = "[7,7,7,7]";
        let s2 = "[7,7,7]";
        assert_eq!(vec![1, 1, 1, 1], get_number_depths(s1));
        assert_eq!(vec![1, 1, 1], get_number_depths(s2));

        // let s1 = "[]";
        // let s2 = "[3]";
        // can't find empty []
    }

    #[test]
    fn depth_and_list_test() {
        let s1 = "[1,1,3,1,1]";
        let s2 = "[1,1,5,1,1]";
        // depth 1
        println!("{:?}", build_depth_list(s1));
        println!("{:?}", build_depth_list(s2));

        let s1 = "[[1],[2,3,4]]";
        let s2 = "[[1],4]";
        // depth 2
        println!("{:?}", build_depth_list(s1));
        println!("{:?}", build_depth_list(s2));

        let s1 = "[9]";
        let s2 = "[[8,7,6]]";
        println!("{:?}", build_depth_list(s1));
        println!("{:?}", build_depth_list(s2));

        let s1 = "[[4,4],4,4]";
        let s2 = "[[4,4],4,4,4]";
        println!("{:?}", build_depth_list(s1));
        println!("{:?}", build_depth_list(s2));

        let s1 = "[7,7,7,7]";
        let s2 = "[7,7,7]";
        println!("{:?}", build_depth_list(s1));
        println!("{:?}", build_depth_list(s2));

        let s1 = "[]";
        let s2 = "[3]";
        println!("{:?}", build_depth_list(s1));
        println!("{:?}", build_depth_list(s2));

        let s1 = "[[[]]]";
        let s2 = "[[]]";
        println!("{:?}", build_depth_list(s1));
        println!("{:?}", build_depth_list(s2));

        let s1 = "[1,[2,[3,[4,[5,6,7]]]],8,9]";
        let s2 = "[1,[2,[3,[4,[5,6,0]]]],8,9]";
        println!("{:?}", build_depth_list(s1));
        println!("{:?}", build_depth_list(s2));
    }

    #[test]
    fn test_symbol_parser() {
        let s1 = "[1,1,3,1,1]";
        let s2 = "[1,1,5,1,1]";
        // depth 1
        println!("{:?}", parse_input(s1));
        println!("{:?}", parse_input(s2));

        let s1 = "[[1],[2,3,4]]";
        let s2 = "[[1],4]";
        // depth 2
        println!("{:?}", parse_input(s1));
        println!("{:?}", parse_input(s2));

        let s1 = "[9]";
        let s2 = "[[8,7,6]]";
        println!("{:?}", parse_input(s1));
        println!("{:?}", parse_input(s2));

        let s1 = "[[4,4],4,4]";
        let s2 = "[[4,4],4,4,4]";
        println!("{:?}", parse_input(s1));
        println!("{:?}", parse_input(s2));

        let s1 = "[7,7,7,7]";
        let s2 = "[7,7,7]";
        println!("{:?}", parse_input(s1));
        println!("{:?}", parse_input(s2));

        let s1 = "[]";
        let s2 = "[3]";
        println!("{:?}", parse_input(s1));
        println!("{:?}", parse_input(s2));

        let s1 = "[[[]]]";
        let s2 = "[[]]";
        println!("{:?}", parse_input(s1));
        println!("{:?}", parse_input(s2));

        let s1 = "[1,[2,[3,[4,[5,6,7]]]],8,9]";
        let s2 = "[1,[2,[3,[4,[5,6,0]]]],8,9]";
        println!("{:?}", parse_input(s1));
        println!("{:?}", parse_input(s2));
    }

    #[test]
    #[ignore = "broken"]
    fn test_symbol_recursive() {
        let s1 = "[1,1,3,1,1]";
        let s2 = "[1,1,5,1,1]";

        let parse1 = parse_input(s1);
        let parse2 = parse_input(s2);
        // depth 1
        println!("{:?}", recurse_symbols(&parse1, 0));
        println!("{:?}", recurse_symbols(&parse2, 0));

        let s1 = "[[1],[2,3,4]]";
        let s2 = "[[1],4]";

        let parse1 = parse_input(s1);
        let parse2 = parse_input(s2);
        println!("{:?}", &parse1);
        println!("{:?}", &parse2);
        // depth 1
        println!("{:?}", recurse_symbols(&parse1, 0));
        println!("{:?}", recurse_symbols(&parse2, 0));
    }

    #[test]
    fn test_stack_version() {
        let s1 = "[1,1,3,1,1]";
        let s2 = "[1,1,5,1,1]";
        let parse1 = parse_input(s1);
        let parse2 = parse_input(s2);
        println!("{:?}", build_symbols_stack(&parse1));
        println!("{:?}", build_symbols_stack(&parse2));


        let s1 = "[[1],[2,3,4]]";
        let s2 = "[[1],4]";
        let parse1 = parse_input(s1);
        let parse2 = parse_input(s2);
        println!("{:?}", build_symbols_stack(&parse1));
        println!("{:?}", build_symbols_stack(&parse2));

        let s1 = "[9]";
        let s2 = "[[8,7,6]]";
        let parse1 = parse_input(s1);
        let parse2 = parse_input(s2);
        println!("{:?}", build_symbols_stack(&parse1));
        println!("{:?}", build_symbols_stack(&parse2));

        let s1 = "[[4,4],4,4]";
        let s2 = "[[4,4],4,4,4]";
        let parse1 = parse_input(s1);
        let parse2 = parse_input(s2);
        println!("{:?}", build_symbols_stack(&parse1));
        println!("{:?}", build_symbols_stack(&parse2));
    }
}
