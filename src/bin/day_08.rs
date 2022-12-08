use std::fs;

fn main() {
    // remember to change the input
    let file = fs::read_to_string("./inputs/day_08_input.txt").unwrap();
    println!("{}", part_1(&file));
    // println!("{}", part_2(&file));
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
    todo!("part 2")
}

fn get_width(input: &str) -> usize {

    for line in input.lines() {
        let split: Vec<&str> = line.split("").filter(|x| !x.is_empty()).collect();
        return split.len();
    }
    0
}

fn get_height(input: &str) -> usize {

    let lines: Vec<&str> = input.lines().collect();
    lines.len()
}

fn get_height_vec(input: &str) -> Vec<u32> {
    let mut num_v: Vec<u32> = Vec::new();
    // let mut line_count = 0;
    for line in input.lines() {
        // line_count += 1;
        // dbg!(line_count);
        let mut split: Vec<u32> = line
            .split("")
            .filter_map(|x| x.parse().ok())
            .collect();
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
    let v: Vec<&Tree> = trees.iter().filter(|x| x.visible).collect();
    v.len()
}

fn calculate_visibility(trees: &mut Vec<Tree>, width: usize, height: usize) {
    // left to right
    for i in 0..width {
        let mut largest = 0;
        for j in 0..height {

            let index = i * width + j;

            //edges
            if i == 0 || i == (width-1) || j==0 || j==(width-1) {
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
    #[ignore = "not ready"]
    fn part_2_works() {
        let result = part_2(&DAY_8_BASIC_INPUT);

        assert_eq!(21, result);
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
