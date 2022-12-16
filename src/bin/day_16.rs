use petgraph::graph::node_index as n;
use petgraph::prelude::*;
use petgraph::visit::depth_first_search;
use petgraph::visit::{Control, DfsEvent};
use std::collections::HashSet;
use std::{collections::BTreeMap, fs};

fn main() {
    let input = fs::read_to_string("./inputs/day_16_input.txt").unwrap();

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
    for (name, valve) in &valves {
        if valve.rate > 0 {
            rates.push(valve.rate);
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
    let mut evaluated: HashSet<String> = HashSet::new();
    recursion(&valves, &mut evaluated, "AA".to_string(), 5);

    99
}

fn recursion(tree: &BTreeMap<String, Valve>, evaluated: &mut HashSet<String>, start: String, depth: u32) {
    if depth == 0 {
        // println!("Depth Reached");
        return;
    }
    if !evaluated.insert(start.clone()) {
        // println!("Already Checked");
        return;
    } 

    println!("Start: {:?}, depth: {}", &start, depth);

    let valve = tree.get(&start).unwrap();
    for name in &valve.neighbours {
        // println!("Name: {:?}", &name);
        
        recursion(&tree, evaluated, name.clone(), depth - 1);
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

#[derive(Debug)]
struct Valve {
    rate: u32,
    neighbours: Vec<String>,
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
    fn part_1_works() {
        assert_eq!(1651, part_1(&BASIC_INPUT_DAY_16));
    }

    #[test]
    fn test_splitting() {
        let s = "DD";
        let split: Vec<&str> = s.split(", ").collect();
        println!("split: {:?}", split);
    }

    #[test]
    fn test_petgraph() {
        let gr: Graph<(), ()> = Graph::from_edges(&[
            (0, 1),
            (0, 2),
            (0, 3),
            (1, 3),
            (2, 3),
            (2, 4),
            (4, 0),
            (4, 5),
        ]);

        // record each predecessor, mapping node → node
        let mut predecessor = vec![NodeIndex::end(); gr.node_count()];
        let start = n(0);
        let goal = n(5);
        depth_first_search(&gr, Some(start), |event| {
            if let DfsEvent::TreeEdge(u, v) = event {
                predecessor[v.index()] = u;
                if v == goal {
                    return Control::Break(v);
                }
            }
            Control::Continue
        });

        let mut next = goal;
        let mut path = vec![next];
        while next != start {
            let pred = predecessor[next.index()];
            path.push(pred);
            next = pred;
        }
        path.reverse();
        println!("path: {:?}", path);
        assert_eq!(&path, &[n(0), n(2), n(4), n(5)]);
    }

    #[test]
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

        println!("graph: {:?}", g);

        println!("DFS");
        let mut dfs = Dfs::new(&g, NodeIndex::new(0));
        while let Some(nx) = dfs.next(&g) {
            println!("{:?}", nx);
            println!("Weight: {:?}",  g.node_weight(nx));
        }
    }
}
