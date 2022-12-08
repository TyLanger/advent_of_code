use std::fs;

fn main() {
    // remember to change the input
    let file = fs::read_to_string("./inputs/day_08_input.txt").unwrap();
    println!("{}", part_1(&file));
    println!("{}", part_2(&file));
}

fn part_1(input: &str) -> usize {
    // need to make a grid of bools
    // calculate visibility
    // count all trues
    // return number

    // should I go line by line?
    // read left to right, keep track of the largest number
    // if a number is bigger, it is visible
    // would prob need to go back and forth
    let width = get_width(input);
    let height = get_height(input);

    let tree_heights = get_height_vec(input);
    let mut trees = create_tree_vec(tree_heights);

    // trees[12].visible = true;

    calculate_visibility(&mut trees, width, height);

    // dbg!(&trees);
    get_num_visible_trees(trees)
}

fn part_2(input: &str) -> u32 {
    let width = get_width(input);
    let height = get_height(input);

    let tree_heights = get_height_vec(input);
    let trees = create_tree_vec(tree_heights);

    find_best_scenic_score(trees, width, height)
}

fn get_width(input: &str) -> usize {
    if let Some(line) = input.lines().next() {
        // for line in input.lines() {
        return line.split("").filter(|x| !x.is_empty()).count();
    }
    0
}

fn get_height(input: &str) -> usize {
    input.lines().count()
}

fn get_height_vec(input: &str) -> Vec<u32> {
    let mut num_v: Vec<u32> = Vec::new();
    // let mut line_count = 0;
    for line in input.lines() {
        // line_count += 1;
        // dbg!(line_count);
        let mut split: Vec<u32> = line.split("").filter_map(|x| x.parse().ok()).collect();
        num_v.append(&mut split);
    }

    num_v
}

fn create_tree_vec(tree_heights: Vec<u32>) -> Vec<Tree> {
    let mut v = Vec::new();
    for height in tree_heights {
        let tree = Tree::new(height);
        v.push(tree);
    }

    v
}

fn get_num_visible_trees(trees: Vec<Tree>) -> usize {
    trees.iter().filter(|x| x.visible).count()
}

fn calculate_visibility(trees: &mut [Tree], width: usize, height: usize) {
    // left to right
    for i in 0..width {
        let mut largest = 0;
        for j in 0..height {
            let index = i * width + j;

            //edges
            if i == 0 || i == (width - 1) || j == 0 || j == (width - 1) {
                trees[index].visible = true;
            }

            if trees[index].height > largest {
                trees[index].visible = true;
                largest = trees[index].height;
            }
        }
    }

    // right to left
    for i in (0..width).rev() {
        let mut largest = 0;
        for j in (0..height).rev() {
            let index = i * width + j;

            if trees[index].height > largest {
                trees[index].visible = true;
                largest = trees[index].height;
            }
        }
    }

    // up to down
    for j in 0..height {
        let mut largest = 0;
        for i in 0..width {
            let index = i * width + j;

            if trees[index].height > largest {
                trees[index].visible = true;
                largest = trees[index].height;
            }
        }
    }

    // down to up
    for j in (0..height).rev() {
        let mut largest = 0;
        for i in (0..width).rev() {
            let index = i * width + j;

            if trees[index].height > largest {
                trees[index].visible = true;
                largest = trees[index].height;
            }
        }
    }
}

fn find_best_scenic_score(trees: Vec<Tree>, width: usize, height: usize) -> u32 {
    let mut best_score = 0;

    // index is left to right

    for i in 1..(width - 1) {
        for j in 1..(height - 1) {
            let index = i * width + j;
            // println!("index: {}", index);
            let my_height = trees[index].height;

            let mut up_view = 1;
            let mut right_view = 1;
            let mut down_view = 1;
            let mut left_view = 1;

            // check up
            for u in 1..i {
                let u_index = (i - u) * width + j;
                // println!("u_index: {}", u_index);
                if u_index < trees.len() {
                    if trees[u_index].height < my_height {
                        up_view += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            // check down
            for u in 1..(height - i - 1) {
                let d_index = (i + u) * width + j;
                // println!("d_index: {}", d_index);
                if d_index < trees.len() {
                    if trees[d_index].height < my_height {
                        down_view += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            // check right
            for u in 1..(width - j - 1) {
                let r_index = index + u;
                // println!("r_index: {}", r_index);
                if r_index < trees.len() {
                    if trees[r_index].height < my_height {
                        right_view += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            // check left
            for u in 1..j {
                let l_index = index - u;
                // println!("l_index: {}", l_index);
                if l_index < trees.len() {
                    if trees[l_index].height < my_height {
                        left_view += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            let score = up_view * right_view * down_view * left_view;
            // println!("({}, {}) = {}  {} {} {} {}", i, j, score, up_view, right_view, down_view, left_view);
            if score > best_score {
                best_score = score;
            }
        }
    }

    best_score
}

#[derive(Clone, Copy, Debug)]
struct Tree {
    height: u32,
    visible: bool,
}

impl Tree {
    fn new(height: u32) -> Self {
        Tree {
            height,
            visible: false,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const DAY_8_BASIC_INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    // #[ignore = "not ready"]
    fn part_1_works() {
        let result = part_1(&DAY_8_BASIC_INPUT);

        assert_eq!(21, result);
    }

    #[test]
    fn part_2_works() {
        let result = part_2(&DAY_8_BASIC_INPUT);

        assert_eq!(8, result);
    }

    #[test]
    fn starting_tree_patch_has_0_visible() {
        let v = vec![Tree::new(0); 25];

        let v2: Vec<&Tree> = v.iter().filter(|x| x.visible).collect();

        assert_eq!(0, v2.len());
    }

    #[test]
    fn one_visible_tree() {
        let mut v = vec![Tree::new(0); 25];

        v[0].visible = true;

        let v2: Vec<&Tree> = v.iter().filter(|x| x.visible).collect();

        assert_eq!(1, v2.len());
    }

    #[test]
    fn split_input_to_numbers() {
        let input = "1234
5678";

        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8], get_height_vec(input));
    }

    #[test]
    fn calculate_width_and_height() {
        let input = "1234
5678";

        assert_eq!(4, get_width(input));
        assert_eq!(2, get_height(input));
    }
}
