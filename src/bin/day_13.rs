use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/day_13_input.txt").unwrap();

    println!("{}", part_1(&input));
}

fn part_1(_input: &str) -> u32 {
    
    
    
    

    99
}

enum Signal {
    List(Vec<Signal>),
    Number(u32),
}

fn in_right_order(v1: &[u32], v2: &[u32]) -> bool {

    for i in 0..v1.len() {
        if v1[i] < v2[i] {
            return true;
        }
    }

    false
}

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
    #[ignore = "not ready yet"]
    fn part_1_works() {
        assert_eq!(13, part_1(&BASIC_INPUT_DAY_13));
    }

    #[test]
    fn get_order_pair_1() {
        let v1 = vec![1,1,3,1,1];
        let v2 = vec![1,1,5,1,1];

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
        let v2 = vec![2,3,4];
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
        let v1 = vec![4,4];
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
    fn get_list_depth() {
        let s1 = "[1,1,3,1,1]";
        let s2 = "[1,1,5,1,1]";
        // depth 1
        assert_eq!(vec![1,1,1,1,1], get_number_depths(s1));
        assert_eq!(vec![1,1,1,1,1], get_number_depths(s2));
        
        let s1 = "[[1],[2,3,4]]";
        let s2 = "[[1],4]";
        // depth 2
        assert_eq!(vec![2,2,2,2], get_number_depths(s1));
        assert_eq!(vec![2,1], get_number_depths(s2));

        let s1 = "[9]";
        let s2 = "[[8,7,6]]";
        assert_eq!(vec![1], get_number_depths(s1));
        assert_eq!(vec![2,2,2], get_number_depths(s2));

        let s1 = "[[4,4],4,4]";
        let s2 = "[[4,4],4,4,4]";
        assert_eq!(vec![2,2,1,1], get_number_depths(s1));
        assert_eq!(vec![2,2,1,1,1], get_number_depths(s2));

        let s1 = "[7,7,7,7]";
        let s2 = "[7,7,7]";
        assert_eq!(vec![1,1,1,1], get_number_depths(s1));
        assert_eq!(vec![1,1,1], get_number_depths(s2));

        let s1 = "[]";
        let s2 = "[3]";
        // can't find empty []
    }


}
