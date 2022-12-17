use petgraph::algo::dijkstra;
// use petgraph::dot::{Config, Dot};
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::prelude::*;
// use petgraph::visit::{Control, DfsEvent};
use std::collections::HashMap;
use std::collections::HashSet;
use std::{collections::BTreeMap, fs};

fn main() {
    let input = fs::read_to_string("./inputs/day_16_input.txt").unwrap();

//     let test = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
// Valve BB has flow rate=13; tunnels lead to valves CC, AA
// Valve CC has flow rate=2; tunnels lead to valves DD, BB
// Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
// Valve EE has flow rate=3; tunnels lead to valves FF, DD
// Valve FF has flow rate=0; tunnels lead to valves EE, GG
// Valve GG has flow rate=0; tunnels lead to valves FF, HH
// Valve HH has flow rate=22; tunnel leads to valve GG
// Valve II has flow rate=0; tunnels lead to valves AA, JJ
// Valve JJ has flow rate=21; tunnel leads to valve II";

//     println!("{}", part_1(&test));

    println!("{}", part_1(&input));
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

    // I want to maximize getting to big valves quickly
    // A* to each valve from the start?
    // pick the one that has the biggest rate * time_left?
    // but maybe it's worth turning a valve on on the way?

    // I want to find a path between all the non zero rates
    // what order is best?

    // depth first search with a cap of 30

    // version 2
    // I only have 30 min
    // what is the best case?
    // highest rate at time = 2
    // 28 min * highest_rate
    // 26 * 2nd rate
    // etc.
    // then test if that is possible given the graph
    // if it is, that's the answer
    // if it isn't try the next best solution

    // I might not open anything at t=2

    // test solution
    // t=2: open 20
    // t=5: open 13 = 33
    // t=9: open 21 = 54
    // t=17: open 22 = 76
    // t=21: open 3 = 79
    // t=24: open 2 = 81

    // 20 * 28 = 560
    //

    // let valve = Valve {
    //     rate: 0,
    //     neighbours: vec!["DD".to_string(), "II".to_string(), "BB".to_string()],
    // };
    // valves.insert("AA".to_string(), valve);

    let valves = parse_into_b_tree_map(&input);
    // this is sorted by the name
    // AA, then BB, then CC, etc.
    // println!("valves: {:?}", valves);

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
    let mut opened: HashSet<String> = HashSet::new();
    let path: Vec<Cave> = Vec::new();
    let mut best = 0;
    // depth 24 or 25 should be all I need to get the right value
    // recursion(&valves, opened, "AA".to_string(), 30, &mut best, 0);
    let mut run_count = 0;

    let dist_lookup = create_dist_lookup(&valves);
    // at 27, it runs 133 times. 1718
    // at 28, it runs 49 times and gets too high
    // at 29, it runs 14 times
    // at 30, it runs 0 times
    let time = 26;
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

fn recursion(
    tree: &BTreeMap<String, Valve>,
    opened: HashSet<String>,
    start: String,
    depth: i32,
    best: &mut u32,
    current_sum: u32,
) {
    // let mut path = path.clone();
    // path.push(Cave::Move(start.clone()));
    if depth == 24 && current_sum == 0 {
        // println!("Quit early if not opening anything");
        return;
    } else if depth == 16 && current_sum < 500 {
        return;
    }

    if depth <= 5 {
        // println!("Depth Reached");

        if current_sum > *best {
            println!("Changed best: {} -> {}", best, current_sum);
            *best = current_sum;
        }
        return;

        // let mut sum = 0;
        // for item in &path {
        //     if let Cave::Open(v) = item {
        //         sum += v;
        //     }
        // }
        // // if sum > 0 {
        // //     println!("Depth reached. Path: {:?} Sum: {:?}", &path, sum);
        // // }
        // if sum > *best {
        //     println!("Depth reached. Path: {:?} Sum: {:?}", &path, sum);
        //     println!("Changed best: {} -> {}", best, sum);
        //     *best = sum;
        // }
        // return;
    }
    // if !evaluated.insert(start.clone()) {
    //     // println!("Already Checked");
    //     // return;
    // }

    // println!("Start: {:?}, depth: {}", &start, depth);

    let valve = tree.get(&start).unwrap();

    for name in &valve.neighbours {
        for i in 0..2 {
            if i == 0 {
                if valve.rate > 0 {
                    let mut new_open = opened.clone();
                    let depth = depth - 1;
                    if !new_open.insert(start.clone()) {
                        // if you try to open the same valve twice
                        return;
                    }
                    // don't open the same valve twice somehow
                    // let mut open_path = path.clone();
                    let value = depth as u32 * valve.rate;
                    // open_path.push(Cave::Open(depth as u32 * valve.rate));
                    let new_sum = current_sum + value;
                    recursion(&tree, new_open, name.clone(), depth - 1, best, new_sum);
                }
            } else {
                recursion(
                    &tree,
                    opened.clone(),
                    name.clone(),
                    depth - 1,
                    best,
                    current_sum,
                );
            }
        }
        // println!("Name: {:?}", &name);
        // if valve.rate > 0 {
        //     // open the valve
        //     path.push(format!("Opened {:?}", start.clone()));
        //     recursion(&tree, evaluated, name.clone(), depth - 1, path.clone());
        // }
    }
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
    // is it running out of thing to visit?

    if time <= 0 {
        // println!("End at time: {}", time);
        if current > *best {
            println!("Changed best: {} -> {}", best, current);
            *best = current;
        }
        *run_count += 1;
        return;
    }

    for (i, item) in non_zero_rates.iter().enumerate() {
        if item != &start {
            let pair = StringPair::new(start.clone(), item.clone());
            let dist = dist_lookup.get(&pair).unwrap();
            let flow = tree.get(item).unwrap().rate;
            
            let new_total = current + ((time as u32)+2) * flow;
            let new_time = time - 1 - *dist as i32;

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

fn calculate_total_flow(path: Vec<Cave>) -> u32 {
    let mut sum = 0;
    for item in &path {
        if let Cave::Open(v) = item {
            sum += v;
        }
    }
    sum
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

#[derive(Clone, Debug)]
enum Cave {
    Move(String),
    Open(u32),
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

    // use std::collections::HashMap;

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
    // #[ignore = "too long"]
    fn part_1_works() {
        // finished in 754.99s
        // got 1710 (wrong)
        // off by 1 is 1732
        assert_eq!(1651, part_1(&BASIC_INPUT_DAY_16));
    }

    #[test]
    #[ignore = "not using this anymore"]
    fn test_total_flow() {
        let path = vec![
            Cave::Move("DD".to_string()),
            Cave::Open(20 * 28),
            Cave::Open(13 * 25),
            Cave::Open(21 * 21),
            Cave::Open(22 * 13),
            Cave::Open(3 * 9),
            Cave::Open(2 * 6),
        ];

        assert_eq!(1651, calculate_total_flow(path));

        let path = vec![
            Cave::Move("DD".to_string()),
            Cave::Open(20 * 29), // 812
            Cave::Open(13 * 26), // 338
            Cave::Open(21 * 22), // 462
            Cave::Open(22 * 14), // 308
            Cave::Open(3 * 10),  // 30
            Cave::Open(2 * 7),   // 14
        ];
        // 1732
        println!("Off by 1 {:?}", calculate_total_flow(path));
    }

    // #[test]
    // #[ignore = "petgraph"]
    // fn test_petgraph() {
    //     let gr: Graph<(), ()> = Graph::from_edges(&[
    //         (0, 1),
    //         (0, 2),
    //         (0, 3),
    //         (1, 3),
    //         (2, 3),
    //         (2, 4),
    //         (4, 0),
    //         (4, 5),
    //     ]);

    //     // record each predecessor, mapping node → node
    //     let mut predecessor = vec![NodeIndex::end(); gr.node_count()];
    //     let start = n(0);
    //     let goal = n(5);
    //     depth_first_search(&gr, Some(start), |event| {
    //         if let DfsEvent::TreeEdge(u, v) = event {
    //             predecessor[v.index()] = u;
    //             if v == goal {
    //                 return Control::Break(v);
    //             }
    //         }
    //         Control::Continue
    //     });

    //     let mut next = goal;
    //     let mut path = vec![next];
    //     while next != start {
    //         let pred = predecessor[next.index()];
    //         path.push(pred);
    //         next = pred;
    //     }
    //     path.reverse();
    //     println!("path: {:?}", path);
    //     assert_eq!(&path, &[n(0), n(2), n(4), n(5)]);
    // }

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
    fn petgraph_edges_as_strings() {
        // maybe it doesn't work with strings?
        // let g = UnGraph::<&str, ()>::from_edges(
        //     &[("AA", "BB"), ("AA", "CC"), ("CC", "DD")]
        // );
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
