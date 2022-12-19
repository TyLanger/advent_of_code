use std::{collections::VecDeque, fs};

fn main() {
    let input = fs::read_to_string("./inputs/day_19_input.txt").unwrap();

    println!("{}", part_1(&input));
}

fn part_1(input: &str) -> u32 {
    let mut blueprints = Vec::new();

    for line in input.lines() {
        let b = Blueprint::from_input(line);

        blueprints.push(b);
    }

    // println!("Blueprints: {:?}", blueprints);

    // what do I need to do?
    // figure out how many geodes I can crack

    // minute 1: 1 ore robot collects 1 ore
    // minute 2: collect 2nd ore
    // minute 3: start building clay robot
    // minute 4: now have 2 ore, 1 clay

    // my choice is a sequence of builds
    // at minute 3, build a clay robot
    // [Ore, None, None, Clay, None, Clay, None, Clay, None, None, None, Obs, Clay, None,
    // None, Obs, None, None, Geode, None, None, Geode, None, None, None]
    // Ore [0]
    // Clay [3, 5, 7, 12]
    // Obs [11, 15]
    // Geo [18, 21]

    // at each minute, check your options
    // then recursively check each permutation?
    // is this just the same as the caves and valves?

    // simulate
    // let mut ore_bots = 1;
    // let mut clay_bots = 0;
    // let mut obs_bots = 0;
    // let mut geode_bots = 0;

    let bots = Bots {
        ore: 1,
        clay: 0,
        obs: 0,
        geode: 0,
    };

    let res = Resources {
        ore: 0,
        clay: 0,
        obs: 0,
    };
    let geodes_cracked: u32 = 0;
    let mut quality = 0;

    for b in blueprints {
        // recursive_bots(&b, bots, res, depth, geodes_cracked, &mut most_cracked);
        let most_cracked = loop_bots_most_geodes(&b, 24);

        println!("for blue: {:?}, cracked: {:?}", b.id, &most_cracked);

        let q = b.id * most_cracked;
        quality += q;

        // for _ in 0..24 {
        //     // build something
        //     // -= resources
        //     let options = b.get_options(&res);
        //     for o in options {
        //         // recursive(o)
        //     }

        //     // gather
        //     res.add(ore_bots, clay_bots, obs_bots);
        //     geodes_cracked += bots.geode;

        //     // finish building
        //     // += bots
        // }

        // for blue: 1, cracked: 6
        // for blue: 2, cracked: 0
        // for blue: 3, cracked: 0
        // for blue: 4, cracked: 0
        // for blue: 5, cracked: 2
        // for blue: 6, cracked: 12
        // for blue: 7, cracked: 2
        // for blue: 8, cracked: 0
        // for blue: 9, cracked: 1
        // for blue: 10, cracked: 0
        // for blue: 11, cracked: 0
        // for blue: 12, cracked: 1
        // for blue: 13, cracked: 9
        // for blue: 14, cracked: 0
        // for blue: 15, cracked: 1
        // for blue: 16, cracked: 4
        // for blue: 17, cracked: 6
        // for blue: 18, cracked: 0
        // for blue: 19, cracked: 10
        // for blue: 20, cracked: 0
        // for blue: 21, cracked: 0
        // for blue: 22, cracked: 2
        // for blue: 23, cracked: 6
        // for blue: 24, cracked: 2
        // for blue: 25, cracked: 11
        // for blue: 26, cracked: 2
        // for blue: 27, cracked: 2
        // for blue: 28, cracked: 3
        // for blue: 29, cracked: 7
        // for blue: 30, cracked: 3
        // 1599
    }

    quality
}

fn part_2(input: &str) -> u32 {
    // do it again, but now in 32 minutes
    let mut blueprints = Vec::new();

    for line in input.lines() {
        let b = Blueprint::from_input(line);

        blueprints.push(b);
    }

    let mut product = 1;
    for b in blueprints.iter().take(3) {
        let most_cracked = loop_bots_most_geodes(&b, 32);
        println!("for blue: {:?}, cracked: {:?}", b.id, &most_cracked);

        product *= most_cracked;
    }

    product
}

// uses too much memory and crashes
fn loop_bots_most_geodes(b: &Blueprint, depth: u32) -> u32 {
    let mut queue: VecDeque<ProblemState> = VecDeque::new();
    let mut most_cracked = 0;

    let problem = ProblemState {
        res: Resources {
            ore: 0,
            clay: 0,
            obs: 0,
        },
        geodes_cracked: 0,
        bots: Bots {
            ore: 1,
            clay: 0,
            obs: 0,
            geode: 0,
        },
        depth,
    };
    queue.push_back(problem);

    // let mut count = 0;
    while !queue.is_empty() {
        // count += 1;
        // if count % 1_000_000 == 0 {
        //     println!("Count: {} queue_len: {:?}", count, &queue.len());
        //     println!("Top queue: {:?}", queue.front());
        //     println!();
        // }
        // Count: 79600000 queue_len: 133740958
        // Top queue: Some(ProblemState { res: Resources { ore: 23, clay: 14, obs: 11 }, geodes_cracked: 0, bots: Bots
        // { ore: 6, clay: 5, obs: 3, geode: 0 }, depth: 2 })

        // Count: 79700000 queue_len: 133899991
        // Top queue: Some(ProblemState { res: Resources { ore: 25, clay: 23, obs: 7 }, geodes_cracked: 0, bots: Bots
        // { ore: 6, clay: 6, obs: 2, geode: 0 }, depth: 2 })

        // Count: 79800000 queue_len: 134073872
        // Top queue: Some(ProblemState { res: Resources { ore: 6, clay: 30, obs: 11 }, geodes_cracked: 0, bots: Bots
        // { ore: 4, clay: 6, obs: 2, geode: 0 }, depth: 2 })
        let current = queue.pop_front().unwrap();
        if current.is_ended() {
            // println!("Ended");
            if current.geodes_cracked > most_cracked {
                most_cracked = current.geodes_cracked;
            }
            continue;
        } else {
            for o in b.get_options(&current.res, &current.bots) {
                let mut current = current;
                current
                    .res
                    .add(current.bots.ore, current.bots.clay, current.bots.obs);
                current.geodes_cracked += current.bots.geode;
                current.depth -= 1;

                match o {
                    Some(bot) => match bot {
                        Bot::Ore => {
                            current.res.ore -= b.ore_cost;
                            current.bots.ore += 1;
                        }
                        Bot::Clay => {
                            current.res.ore -= b.clay_cost;
                            current.bots.clay += 1;
                        }
                        Bot::Obs => {
                            current.res.ore -= b.obs_cost;
                            current.res.clay -= b.obs_clay_cost;
                            current.bots.obs += 1;
                        }
                        Bot::Geode => {
                            current.res.ore -= b.geode_cost;
                            current.res.obs -= b.geode_obs_cost;
                            current.bots.geode += 1;
                        }
                    },
                    _ => {}
                }

                queue.push_back(current);
            }
        }
    }

    most_cracked
}

fn recursive_bots(
    blue: &Blueprint,
    bots: Bots,
    res: Resources,
    depth: u32,
    geodes_cracked: u32,
    most_cracked: &mut u32,
) {
    let mut geodes_cracked = geodes_cracked;

    if geodes_cracked > *most_cracked {
        println!(
            "Better: {:?} -> {:?} at depth: {:?}",
            most_cracked, geodes_cracked, depth
        );
        println!("  With bots: {:?}", bots);
        *most_cracked = geodes_cracked;
    }

    if depth == 0 {
        return;
    }

    for o in blue.get_options(&res, &bots) {
        let mut res = res;
        let mut bots = bots;
        res.add(bots.ore, bots.clay, bots.obs);
        geodes_cracked += bots.geode;
        match o {
            None => {
                recursive_bots(blue, bots, res, depth - 1, geodes_cracked, most_cracked);
            }
            Some(bot) => match bot {
                Bot::Ore => {
                    res.ore -= blue.ore_cost;
                    bots.ore += 1;
                    recursive_bots(blue, bots, res, depth - 1, geodes_cracked, most_cracked);
                }
                Bot::Clay => {
                    res.ore -= blue.clay_cost;
                    bots.clay += 1;
                    recursive_bots(blue, bots, res, depth - 1, geodes_cracked, most_cracked);
                }
                Bot::Obs => {
                    res.ore -= blue.obs_cost;
                    res.clay -= blue.obs_clay_cost;
                    bots.obs += 1;
                    recursive_bots(blue, bots, res, depth - 1, geodes_cracked, most_cracked);
                }
                Bot::Geode => {
                    res.ore -= blue.geode_cost;
                    res.obs -= blue.geode_obs_cost;
                    bots.geode += 1;
                    recursive_bots(blue, bots, res, depth - 1, geodes_cracked, most_cracked);
                }
            },
        }
    }
}

fn get_naive_geodes_cracked(blue: &Blueprint, depth: u32) -> Vec<u32> {
    // assume 1 of each
    let mut v = Vec::new();

    let mut bots = Bots {
        ore: 1,
        clay: 0,
        obs: 0,
        geode: 0,
    };
    let mut res = Resources {
        ore: 0,
        clay: 0,
        obs: 0,
    };

    let mut geodes_cracked = 0;

    for i in 0..depth {
        let mut to_build = None;
        // choose
        // -= res
        if bots.clay == 0 && blue.can_build(Bot::Clay, &res) {
            to_build = Some(Bot::Clay);
        } else if bots.obs == 0 && blue.can_build(Bot::Obs, &res) {
            to_build = Some(Bot::Obs);
        } else if bots.geode == 0 && blue.can_build(Bot::Geode, &res) {
            to_build = Some(Bot::Geode);
        }

        // gather
        // += res
        res.add(bots.ore, bots.clay, bots.obs);
        geodes_cracked += bots.geode;
        v.push(geodes_cracked);

        // build
        // += bots
        match to_build {
            Some(bot) => match bot {
                Bot::Ore => bots.ore += 1,
                Bot::Clay => bots.clay += 1,
                Bot::Obs => bots.obs += 1,
                Bot::Geode => bots.geode += 1,
            },
            None => {}
        }
    }
    v
}

#[derive(Debug, Clone, Copy)]
struct ProblemState {
    res: Resources,
    geodes_cracked: u32,
    bots: Bots,
    depth: u32,
}

impl ProblemState {
    fn get_depth(&self) -> u32 {
        self.depth
    }

    fn is_ended(&self) -> bool {
        self.depth == 0
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Bot {
    Ore,
    Clay,
    Obs,
    Geode,
}

#[derive(Debug, Clone, Copy)]
struct Bots {
    ore: u32,
    clay: u32,
    obs: u32,
    geode: u32,
}

#[derive(Debug, Clone, Copy)]
struct Resources {
    ore: u32,
    clay: u32,
    obs: u32,
}

impl Resources {
    fn add(&mut self, ore: u32, clay: u32, obs: u32) {
        self.ore += ore;
        self.clay += clay;
        self.obs += obs;
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Blueprint {
    id: u32,
    ore_cost: u32,
    clay_cost: u32,
    obs_cost: u32,
    obs_clay_cost: u32,
    geode_cost: u32,
    geode_obs_cost: u32,
}

impl Blueprint {
    fn from_input(input: &str) -> Blueprint {
        let split: Vec<&str> = input.split(&[' ', ':']).collect();
        // println!("line split: {:?}", split);
        let id = split[1].parse().unwrap();
        let ore_cost = split[7].parse().unwrap();
        let clay_cost = split[13].parse().unwrap();
        let obs_cost = split[19].parse().unwrap();
        let obs_clay_cost = split[22].parse().unwrap();
        let geode_cost = split[28].parse().unwrap();
        let geode_obs_cost = split[31].parse().unwrap();

        Blueprint {
            id,
            ore_cost,
            clay_cost,
            obs_cost,
            obs_clay_cost,
            geode_cost,
            geode_obs_cost,
        }
    }

    fn can_build(&self, bot: Bot, res: &Resources) -> bool {
        match bot {
            Bot::Ore => res.ore >= self.ore_cost,
            Bot::Clay => res.ore >= self.clay_cost,
            Bot::Obs => res.ore >= self.obs_cost && res.clay >= self.obs_clay_cost,
            Bot::Geode => res.ore >= self.geode_cost && res.obs >= self.geode_obs_cost,
        }
    }

    fn get_options(&self, res: &Resources, bots: &Bots) -> Vec<Option<Bot>> {
        let mut v = Vec::new();

        // if you can make a geode bot, do
        if self.can_build(Bot::Geode, res) {
            v.push(Some(Bot::Geode));
            return v;
        }

        // if have 0 obs, build 1
        // probaby true
        if bots.obs == 0 && self.can_build(Bot::Obs, res) {
            v.push(Some(Bot::Obs));
            return v;
        }

        // might not be true here
        // if stuff only needs 5 clay, might be better to build a second ore bot
        // if bots.clay == 0 && self.can_build(Bot::Clay, res) {
        //     v.push(Some(Bot::Clay));
        //     return v;
        // }

        // don't build ore if the ore_bots you have already
        // can build anything in 1 turn
        if self.can_build(Bot::Ore, res) && res.ore < 10 {
            v.push(Some(Bot::Ore));
        }
        if self.can_build(Bot::Clay, res) && res.clay < self.obs_clay_cost * 2 {
            v.push(Some(Bot::Clay));
        }
        if self.can_build(Bot::Obs, res) && res.obs < self.geode_obs_cost * 2 {
            v.push(Some(Bot::Obs));
        }

        // if you have a bunch of resources, need to build something.

        if res.ore < 6 {
            v.push(None);
        }

        v
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const BASIC_INPUT_DAY_19: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    #[ignore = "long"]
    fn part_1_works() {
        assert_eq!(33, part_1(&BASIC_INPUT_DAY_19));
    }

    #[test]
    fn part_2_works() {
        assert_eq!(56 * 62, part_2(&BASIC_INPUT_DAY_19));
    }

    #[test]
    fn test_can_build() {
        let res = Resources {
            ore: 4,
            clay: 14,
            obs: 7,
        };
        let blue = Blueprint {
            id: 0,
            ore_cost: 4,
            clay_cost: 2,
            obs_cost: 3,
            obs_clay_cost: 14,
            geode_cost: 2,
            geode_obs_cost: 7,
        };

        assert_eq!(true, blue.can_build(Bot::Ore, &res));
        assert_eq!(true, blue.can_build(Bot::Clay, &res));
        assert_eq!(true, blue.can_build(Bot::Obs, &res));
        assert_eq!(true, blue.can_build(Bot::Geode, &res));

        // missing both
        let res = Resources {
            ore: 3,
            clay: 13,
            obs: 6,
        };
        assert_eq!(false, blue.can_build(Bot::Ore, &res));
        assert_eq!(true, blue.can_build(Bot::Clay, &res));
        assert_eq!(false, blue.can_build(Bot::Obs, &res));
        assert_eq!(false, blue.can_build(Bot::Geode, &res));

        let res = Resources {
            ore: 0,
            clay: 13,
            obs: 6,
        };
        assert_eq!(false, blue.can_build(Bot::Ore, &res));
        assert_eq!(false, blue.can_build(Bot::Clay, &res));
        assert_eq!(false, blue.can_build(Bot::Obs, &res));
        assert_eq!(false, blue.can_build(Bot::Geode, &res));

        // missing 1
        let res = Resources {
            ore: 10,
            clay: 13,
            obs: 6,
        };
        assert_eq!(true, blue.can_build(Bot::Ore, &res));
        assert_eq!(true, blue.can_build(Bot::Clay, &res));
        assert_eq!(false, blue.can_build(Bot::Obs, &res));
        assert_eq!(false, blue.can_build(Bot::Geode, &res));
    }

    #[test]
    fn test_options() {
        let res = Resources {
            ore: 4,
            clay: 14,
            obs: 0,
        };
        let blue = Blueprint {
            id: 0,
            ore_cost: 4,
            clay_cost: 2,
            obs_cost: 3,
            obs_clay_cost: 14,
            geode_cost: 2,
            geode_obs_cost: 7,
        };
        let bots = Bots {
            ore: 1,
            clay: 1,
            obs: 1,
            geode: 1,
        };

        let exp = vec![Some(Bot::Ore), Some(Bot::Clay), Some(Bot::Obs), None];
        assert_eq!(exp, blue.get_options(&res, &bots));

        let res = Resources {
            ore: 0,
            clay: 13,
            obs: 6,
        };
        let exp = vec![None];
        assert_eq!(exp, blue.get_options(&res, &bots));

        // if you can build a geode, do
        let res = Resources {
            ore: 4,
            clay: 14,
            obs: 7,
        };
        let exp = vec![Some(Bot::Geode)];
        assert_eq!(exp, blue.get_options(&res, &bots));

        // can't build nothing if have 6 ore
        let res = Resources {
            ore: 6,
            clay: 14,
            obs: 5,
        };
        let exp = vec![Some(Bot::Ore), Some(Bot::Clay), Some(Bot::Obs)];
        assert_eq!(exp, blue.get_options(&res, &bots));
    }

    #[test]
    fn test_naive_geodes() {
        let blue = Blueprint {
            id: 0,
            ore_cost: 4,
            clay_cost: 2,
            obs_cost: 3,
            obs_clay_cost: 14,
            geode_cost: 2,
            geode_obs_cost: 7,
        };

        let naive = get_naive_geodes_cracked(&blue, 24);
        println!("naive: {:?}", naive);
        let naive = get_naive_geodes_cracked(&blue, 32);
        println!("naive: {:?}", naive);
    }
}
