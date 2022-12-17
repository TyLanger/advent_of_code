use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::prelude::*;
use std::collections::HashMap;
use std::{collections::BTreeMap, fs};

fn main() {
    let input = fs::read_to_string("./inputs/day_16_input.txt").unwrap();

    println!("{}", part_1(&input)); // 1820
    println!("{}", part_2(&input)); // 1820
}

fn part_1(input: &str) -> u32 {
    // this is impossible
    // Is this a travelling salesman?

    // there are only 15 working valves in the input
    // in the test input, all valves are opened within time.
    // it's maybe possible in the real input

    // there are 50 lines, 50 valves
    // at AA, you have 5 choices

    // in the test
    // AA has 3 choices: DD, II, BB
    // DD has CC, AA, EE
    // II has AA, JJ
    // BB has CC, AA

    // I want to find a path between all the non zero rates
    // what order is best?

    // depth first search with a cap of 30

    // test solution
    // t=2: open 20
    // t=5: open 13 = 33
    // t=9: open 21 = 54
    // t=17: open 22 = 76
    // t=21: open 3 = 79
    // t=24: open 2 = 81

    // 20 * 28 = 560

    let valves = parse_into_b_tree_map(&input);
    // this is sorted by the name
    // AA, then BB, then CC, etc.

    let mut rates = Vec::new();
    let mut non_zero_rates = Vec::new();

    for (name, valve) in &valves {
        if valve.rate > 0 {
            rates.push(valve.rate);
            non_zero_rates.push(name.clone());
        }
    }
    rates.sort_by(|a, b| b.cmp(&a));
    println!("Non-zero rates sorted: {:?}", rates);
    // Non-zero rates sorted: [22, 21, 20, 13, 3, 2] = 6
    // Non-zero rates sorted: [22, 21, 20, 19, 18, 17, 16, 15, 13, 11, 10, 9, 7, 6, 5] = 15
    // orders is n!
    // 720
    // 15! = 1.3076744e+12 1 trillion

    // I could use A* to get the length from the start for each
    // running straight there, it would have a cost(length) and I could calc the total flow
    // 20 is AA -> DD = 30-2 = 28 * 20 = 560

    // depth first
    // can't open the same valve twice
    let mut best = 0;
    // depth 24 or 25 should be all I need to get the right value
    // recursion(&valves, opened, "AA".to_string(), 30, &mut best, 0);
    let mut run_count = 0;

    let dist_lookup = create_dist_lookup(&valves);

    let time = 30;
    recursion_with_dist_lookup(
        &valves,
        &dist_lookup,
        non_zero_rates,
        "AA".to_string(),
        time,
        0,
        &mut best,
        &mut run_count,
    );

    println!("Runs: {}", run_count);

    best
}

fn part_2(input: &str) -> u32 {
    // take 4 min to teach the elephant at the start

    // what changes here?
    // can open 2 valves at a time.
    // need a path for me and a path for the elephant.
    // both go to ~half of them.

    // pick a random set of valves to go to.
    // give to one. Give the rest to the other.
    // calculate the best order for this smaller subset
    // using the same logic as part 1
    // then check every combination I could give to each of them.

    // I open valves AA, BB, CC
    // El opens valves DD, EE, FF
    // get optimal order
    // save result
    // Then try for I open AA, BB, DD
    // el opens CC, EE, FF
    // me opening DD, EE, FF
    // el opening AA, BB, CC
    // is the same as case 1

    // if there were only 2 valves, we'd each take 1
    // if there were 3, with dists: A=1, B=2, c=4
    // one does A, B. Other does C
    // maybe not. B would need to be 2 from A

    // how many ways can I split the input?
    // there are 15 'real' valves
    // with [A, B, C, D], there is
    // [A, B] && [C, D]
    // [A, C] && [B, D]
    // [A, D] && [B, C]
    // [A] && [B, C, D]
    // [B] && [A, C, D]
    // [C] && [A, B, D]
    // [D] && [A, B, C]
    // 7 combinations at least

    // with 15, it's maybe half of ~32_000?
    // with 15 items, that's a 15digit binary number (~32_000)
    // ex. 1001000101010
    //  each 1 is a visit for me. each 0 is a visit for the elephant
    // for i in 0..(0b111_1111_1111_1111 / 2)
    // this is the unique combinations

    // version 2
    // choose 2 at each point
    // part 1 path:
    // AA -> CC -> EE -> JJ, etc
    // part 2
    // AA -> CC,EE -> JJ,DD, etc

    // for (i, item) in non_zero_rates.iter().enumerate() {
    // for (j, item) in i..len() {

    // how to find the right split?
    // non_zero_valves = v[name; 15]
    // split at 7
    // run
    // split at 6
    // run
    // it's probably fastest to split in half
    // or rather probably best to both finish at the same time.
    // want close to same length paths

    let valves = parse_into_b_tree_map(&input);
    // this is sorted by the name
    // AA, then BB, then CC, etc.

    let mut rates = Vec::new();
    let mut non_zero_rates = Vec::new();

    for (name, valve) in &valves {
        if valve.rate > 0 {
            rates.push(valve.rate);
            non_zero_rates.push(name.clone());
        }
    }
    rates.sort_by(|a, b| b.cmp(&a));
    println!("Non-zero rates sorted: {:?}", rates);
    // Non-zero rates sorted: [22, 21, 20, 13, 3, 2] = 6
    // Non-zero rates sorted: [22, 21, 20, 19, 18, 17, 16, 15, 13, 11, 10, 9, 7, 6, 5] = 15

    let mut run_count = 0;

    let dist_lookup = create_dist_lookup(&valves);

    let time = 26;

    let len = non_zero_rates.len();
    let half_len = len / 2;
    let my_visits: Vec<String> = non_zero_rates[0..half_len].to_vec();
    let elephant_visits: Vec<String> = non_zero_rates[half_len..len].to_vec();
    println!("my visits: {:?}", &my_visits);
    println!("elephant visits: {:?}", &elephant_visits);

    // optimal is [DD, HH, EE] [JJ, BB, CC]
    let cheat_1 = vec!["DD".to_string(), "HH".to_string(), "EE".to_string()];
    let cheat_2 = vec!["JJ".to_string(), "BB".to_string(), "CC".to_string()];

    let mut my_best = 0;
    let mut el_best = 0;
    recursion_with_dist_lookup(
        &valves,
        &dist_lookup,
        cheat_1,
        "AA".to_string(),
        time,
        0,
        &mut my_best,
        &mut run_count,
    );
    recursion_with_dist_lookup(
        &valves,
        &dist_lookup,
        cheat_2,
        "AA".to_string(),
        time,
        0,
        &mut el_best,
        &mut run_count,
    );

    println!("Runs: {}", run_count);

    my_best + el_best
}

fn recursion_with_dist_lookup(
    tree: &BTreeMap<String, Valve>,
    dist_lookup: &HashMap<StringPair, u32>,
    non_zero_rates: Vec<String>,
    start: String,
    time: i32,
    current: u32,
    best: &mut u32,
    run_count: &mut u32,
) {
    // bug was here
    // used to be if time < 0
    // then update best
    // if I update every time, it works

    if current > *best {
        println!("Changed best: {} -> {}", best, current);
        *best = current;
    }
    *run_count += 1;
    if time <= 0 {
        return;
    }

    for (i, item) in non_zero_rates.iter().enumerate() {
        if item != &start {
            let pair = StringPair::new(start.clone(), item.clone());
            let dist = dist_lookup.get(&pair).unwrap();

            let new_time = time - 1 - *dist as i32;
            // is the valve too far away to get to?
            if new_time < 0 {
                // can't reach this valve
                if current > *best {
                    println!("Changed best interior: {} -> {}", best, current);
                    *best = current;
                }
                *run_count += 1;
                continue;
            }

            let flow = tree.get(item).unwrap().rate;
            let new_total = current + (new_time as u32) * flow;

            let mut to_visit = non_zero_rates.clone();
            to_visit.remove(i);

            recursion_with_dist_lookup(
                tree,
                dist_lookup,
                to_visit,
                item.clone(),
                new_time,
                new_total,
                best,
                run_count,
            );
        }
    }
}

fn parse_into_b_tree_map(input: &str) -> BTreeMap<String, Valve> {
    let mut valves = BTreeMap::new();

    for line in input.lines() {
        let name = line.split(' ').take(2).skip(1).next().unwrap();
        let flow = line
            .split_once("rate=")
            .unwrap()
            .1
            .split_once(';')
            .unwrap()
            .0
            .parse::<u32>()
            .unwrap();

        let neighbours: Vec<String> = line
            .split_once("valve")
            .unwrap()
            .1
            .split_once(' ')
            .unwrap()
            .1
            .split(", ")
            .map(|x| x.to_string())
            .collect();

        // let neighbours: Vec<&str> = line.split_once("valves ").unwrap().1.split(", ").collect();
        // println!("name: {:?} flow: {} n: {:?}", name, flow, neighbours);

        let v = Valve {
            rate: flow,
            neighbours,
        };
        valves.insert(name.to_string(), v);
    }

    valves
}

fn create_dist_lookup(tree: &BTreeMap<String, Valve>) -> HashMap<StringPair, u32> {
    let mut names = Vec::new();

    for (name, _valve) in tree {
        names.push(name.clone());
    }

    let mut edges = Vec::new();
    for (tree_i, (_name, valve)) in tree.iter().enumerate() {
        for other_name in &valve.neighbours {
            // let b = g.add_node(other_name.clone());
            // let index = names.find(&other_name).unwrap();
            let mut index = 0;
            for i in 0..names.len() {
                if &&names[i] == &other_name {
                    index = i;
                    break;
                }
            }

            edges.push((NodeIndex::new(tree_i), NodeIndex::new(index)));
            // g.add_edge(NodeIndex::new(tree_i), NodeIndex::new(index), 1);
        }
    }
    let g: Graph<i32, (), Undirected> = UnGraph::<i32, ()>::from_edges(&edges);

    let mut dist_lookup = HashMap::new();

    for (i, (name, valve)) in tree.iter().enumerate() {
        if valve.rate > 0 || name == "AA" {
            // calcualte the path to it
            let node_map = dijkstra(&g, NodeIndex::new(i), None, |_| 1);

            for (index, dist) in node_map {
                let a = name.clone();
                let b = names[index.index()].clone();

                let rate = tree.get(&b).unwrap().rate;

                if a != b && rate > 0 {
                    let pair = StringPair::new(a, b);
                    dist_lookup.insert(pair, dist);
                }
            }
        }
    }

    dist_lookup
}

#[derive(Debug)]
struct Valve {
    rate: u32,
    neighbours: Vec<String>,
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
struct StringPair {
    a: String,
    b: String,
}

impl StringPair {
    fn new(a: String, b: String) -> Self {
        if a < b {
            StringPair { a, b }
        } else {
            StringPair { a: b, b: a }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const BASIC_INPUT_DAY_16: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    #[ignore = "long"]
    fn part_1_works() {
        assert_eq!(1651, part_1(&BASIC_INPUT_DAY_16));
    }

    #[test]
    // #[ignore = "too long"]
    fn part_2_works() {
        assert_eq!(1707, part_2(&BASIC_INPUT_DAY_16));
    }

    #[test]
    #[ignore = "visual"]
    fn undirected_graph() {
        let mut g = UnGraph::<&str, i32>::new_undirected();
        let a = g.add_node("AA");
        let b = g.add_node("BB");
        let c = g.add_node("CC");
        let d = g.add_node("DD");
        let e = g.add_node("EE");

        g.add_edge(a, b, 1);
        g.add_edge(a, c, 1);
        g.add_edge(c, d, 1);
        g.add_edge(d, e, 1);
        g.add_edge(b, e, 1);
        // g.extend_with_edges(&[
        //     (a, b), (a, c),
        //     (c, d)

        // ]);

        println!("graph: {:?}", g);

        let mut dfs = Dfs::new(&g, a);
        while let Some(nx) = dfs.next(&g) {
            println!("{:?}", nx);
        }
    }

    #[test]
    #[ignore = "visual"]
    fn input_to_graph() {
        let tree = parse_into_b_tree_map(&BASIC_INPUT_DAY_16);

        let mut g = UnGraph::<String, i32>::new_undirected();

        let mut names = Vec::new();

        for (name, _valve) in &tree {
            names.push(name.clone());

            g.add_node(name.clone());
        }
        for (tree_i, (_name, valve)) in tree.iter().enumerate() {
            for other_name in &valve.neighbours {
                // let b = g.add_node(other_name.clone());
                // let index = names.find(&other_name).unwrap();
                let mut index = 0;
                for i in 0..names.len() {
                    if &&names[i] == &other_name {
                        index = i;
                    }
                }

                g.add_edge(NodeIndex::new(tree_i), NodeIndex::new(index), 1);
            }
        }

        println!("names: {:?}", &names);
        println!("graph: {:?}", g);

        println!("DFS");
        let mut dfs = Dfs::new(&g, NodeIndex::new(0));
        while let Some(nx) = dfs.next(&g) {
            println!("{:?}", nx);
            println!("Weight: {:?}", g.node_weight(nx));
        }
    }

    #[test]
    #[ignore = "visual"]
    fn petgraph_dijkstra() {
        let g: Graph<i32, (), Undirected> =
            UnGraph::<i32, ()>::from_edges(&[(1, 2), (2, 3), (3, 4), (1, 4)]);

        let _node_map = dijkstra(&g, 1.into(), Some(4.into()), |_| 1);
        // println!("path? {:?}", node_map);

        // a ----> b ----> e ----> f
        // ^       |       ^       |
        // |       v       |       v
        // d <---- c       h <---- g
        // let expected_res: HashMap<NodeIndex, usize> = [
        //     (a, 3),
        //     (b, 0),
        //     (c, 1),
        //     (d, 2),
        //     (e, 1),
        //     (f, 2),
        //     (g, 3),
        //     (h, 4),
        // ]
        // .iter()
        // .cloned()
        // .collect();
        // let res = dijkstra(&graph, b, None, |_| 1);
        // start at b, to none
        // will calulate the distance from b to every other node
        // b to a is 3. it has to go around the square

        // A - B
        //  \
        //   C - D
        let names = vec!["AA", "BB", "CC", "DD"];
        let g: Graph<i32, (), Undirected> =
            UnGraph::<i32, ()>::from_edges(&[(0, 1), (0, 2), (2, 3)]);

        let node_map = dijkstra(&g, 0.into(), None, |_| 1);
        for (index, item) in node_map {
            println!(
                "{:?} to {:?} weight: {}",
                names[0],
                names[index.index()],
                item
            );
        }
        let node_map = dijkstra(&g, 1.into(), None, |_| 1);
        for (index, item) in node_map {
            println!(
                "{:?} to {:?} weight: {}",
                names[1],
                names[index.index()],
                item
            );
        }
    }

    #[test]
    fn test_string_pairs() {
        // will be used to get a dist from a lookup
        // and avoid duplicates
        let s1 = StringPair::new("AA".to_string(), "BB".to_string());
        let s2 = StringPair::new("BB".to_string(), "AA".to_string());

        assert_eq!(s1, s2);
    }

    #[test]
    fn parse_input_to_relevent_distances() {
        let tree = parse_into_b_tree_map(&BASIC_INPUT_DAY_16);

        let dist_lookup = create_dist_lookup(&tree);

        println!("dist_lookup: {:?} len: {}", &dist_lookup, dist_lookup.len());

        // AA has rate 0, but it's the start so don't skip it.
        let pair = StringPair::new("BB".to_string(), "AA".to_string());
        assert_eq!(Some(&1), dist_lookup.get(&pair));

        // II has rate 0. Skip it
        let pair = StringPair::new("AA".to_string(), "II".to_string());
        assert_eq!(None, dist_lookup.get(&pair));
    }
}
