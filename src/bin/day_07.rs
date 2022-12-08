use std::{cell::RefCell, fmt::Display, fs, rc::Rc};

fn main() {
    // remember to change the input
    let file = fs::read_to_string("./inputs/day_07_input.txt").unwrap();
    println!("{}", part_1(&file));
    println!("{}", part_2(&file));
}

fn part_1(input: &str) -> u32 {
    // construct a tree from given input
    // find all directories that have <100k memory
    // - a (dir)
    //  - e (dir)
    //      - file 500
    //  - file 1000
    // return their sums.
    // a and e
    // 1000 + 500 + 500

    // have a root
    // any number of dirs and files

    // vec of all dirs
    // node has a vec of files and a vec of dir children

    // order
    // start at root
    // insert each file
    // insert each node (without their files)
    // go into one of the nodes
    // update its files
    // go back
    // go into the other node
    // update its files

    let root = build_tree(input);

    // println!("root to_string: {:?}", root.borrow().to_string());
    let v = root.borrow().get_children_sizes_under(100000);

    let mut size = 0;
    for item in v {
        size += item;
    }
    size
}

fn part_2(input: &str) -> u32 {
    let root = build_tree(input);

    let total_size = 70000000;
    let needed = 30000000;

    let root_size = root.borrow().get_size();

    // sample input
    // 700 - 483 = 216
    // need 300
    // find dirs that are bigger than 300 - 216
    // find the smallest from that list

    let unused = total_size - root_size;
    let to_delete = needed - unused;

    let mut v = root.borrow().get_children_sizes_over(to_delete);
    // find the smallest
    v.sort();

    v[0]
}

fn build_tree(input: &str) -> Rc<RefCell<TreeNode>> {
    let root = Rc::new(RefCell::new(TreeNode::new()));
    let mut current = Rc::clone(&root);

    let lines = input.lines().skip(1);
    for line in lines {
        if line.contains("$ cd") {
            // name
            // ..

            let arg = line.split_once("cd ").unwrap().1;

            if arg.contains("..") {
                let current_clone = Rc::clone(&current);
                current = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
            } else {
                let current_clone = Rc::clone(&current);
                current = Rc::clone(&current_clone.borrow().get_dir_by_name(arg).unwrap());
            }
        } else if line.contains("$ ls") {
            // next few lines will be files and dirs
        } else if line.contains("dir") {
            let name = line.split_once("dir ").unwrap().1;

            let mut new_node = TreeNode::new();
            new_node.name(name);
            let child = Rc::new(RefCell::new(new_node));
            child.borrow_mut().parent = Some(Rc::clone(&current));
            add_child_dir(Rc::clone(&current), child);
        } else {
            // 12312398 file.txt
            let size = line.split_once(' ').unwrap().0;
            let value = size.parse().unwrap();

            add_child_file(Rc::clone(&current), value);
        }
    }

    root
}

fn add_child_file(current: Rc<RefCell<TreeNode>>, value: u32) {
    current.borrow_mut().files.push(value);
}

fn add_child_dir(current: Rc<RefCell<TreeNode>>, child: Rc<RefCell<TreeNode>>) {
    current.borrow_mut().children.push(Rc::clone(&child));
}

// maybe this is what I need
// https://applied-math-coding.medium.com/a-tree-structure-implemented-in-rust-8344783abd75
struct TreeNode {
    files: Vec<u32>,
    name: String,
    children: Vec<Rc<RefCell<TreeNode>>>,
    parent: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new() -> Self {
        TreeNode {
            files: vec![],
            name: "".to_string(),
            children: vec![],
            parent: None,
        }
    }

    fn name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn get_size(&self) -> u32 {
        let mut self_count = 0;
        for f in &self.files {
            self_count += f;
        }

        let mut child_count = 0;
        for c in &self.children {
            child_count += c.borrow().get_size();
        }

        self_count + child_count
    }

    fn get_children_sizes_under(&self, under: u32) -> Vec<u32> {
        let mut v = vec![];

        for c in &self.children {
            let size: u32 = c.borrow().get_size();
            if size < under {
                v.push(size);
            }

            let v_children = c.borrow().get_children_sizes_under(under);

            for item in v_children {
                v.push(item);
            }
        }

        v
    }

    fn get_children_sizes_over(&self, over: u32) -> Vec<u32> {
        let mut v = vec![];

        for c in &self.children {
            let size: u32 = c.borrow().get_size();
            if size > over {
                v.push(size);
            }

            let v_children = c.borrow().get_children_sizes_over(over);

            for item in v_children {
                v.push(item);
            }
        }

        v
    }

    fn get_dir_by_name(&self, name: &str) -> Option<Rc<RefCell<TreeNode>>> {
        for c in &self.children {
            if c.borrow().name == name {
                return Some(Rc::clone(c));
            }
        }
        None
    }
}

impl Display for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // this doesn't look pretty
        // but it was useful for debugging once
        let mut output = "".to_string();
        for f in &self.files {
            // the newlines don't work right
            output = format!("{}file {}\n", output, f);
        }

        for c in &self.children {
            output = format!("{}child: {}\n", output, c.borrow());
        }

        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const DAY_7_BASIC_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn part_1_works() {
        let result = part_1(&DAY_7_BASIC_INPUT);

        assert_eq!(95437, result);
    }

    #[test]
    fn part_2_works() {
        let result = part_2(&DAY_7_BASIC_INPUT);

        assert_eq!(24933642, result);
    }

    #[test]
    fn empty_tree_size_is_0() {
        let node = TreeNode::new();

        assert_eq!(0, node.get_size());
    }

    #[test]
    fn init_tree() {
        let root = Rc::new(RefCell::new(TreeNode::new()));
        let current = Rc::clone(&root);

        add_child_file(current, 14);

        assert_eq!(14, root.borrow().get_size());
    }

    #[test]
    fn create_tree_with_children() {
        let root = Rc::new(RefCell::new(TreeNode::new()));

        let mut current = Rc::clone(&root);
        add_child_file(current, 1);
        current = Rc::clone(&root);
        add_child_file(current, 2);

        assert_eq!(3, root.borrow().get_size());

        let child = Rc::new(RefCell::new(TreeNode::new()));
        current = Rc::clone(&child);

        add_child_file(current, 3);
        current = Rc::clone(&root);
        add_child_dir(current, child);

        assert_eq!(6, root.borrow().get_size());
    }

    #[test]
    fn add_x_children_with_files() {
        let root = Rc::new(RefCell::new(TreeNode::new()));

        let mut current;
        for i in 1..=3 {
            current = Rc::clone(&root);
            let child = Rc::new(RefCell::new(TreeNode::new()));
            let child_clone = Rc::clone(&child);
            add_child_dir(current, child);
            add_child_file(child_clone, i);
        }

        assert_eq!(6, root.borrow().get_size());
    }

    #[test]
    fn depth_first_file_insertion() {
        // root
        // dir a
        //    file 1
        //    file 2
        // dir b
        //    file 3
        //    file 4

        let root = Rc::new(RefCell::new(TreeNode::new()));

        let mut node_a = TreeNode::new();
        node_a.name("a");
        let child_a = Rc::new(RefCell::new(node_a));
        child_a.borrow_mut().parent = Some(Rc::clone(&root));
        add_child_dir(Rc::clone(&root), Rc::clone(&child_a));

        let mut node_b = TreeNode::new();
        node_b.name("b");
        let child_b = Rc::new(RefCell::new(node_b));
        child_b.borrow_mut().parent = Some(Rc::clone(&root));
        add_child_dir(Rc::clone(&root), child_b);

        // add files to a
        let mut current = Rc::clone(&child_a);
        add_child_file(Rc::clone(&current), 1);
        add_child_file(Rc::clone(&current), 2);

        let current_clone = Rc::clone(&current);
        current = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());

        let current_clone = Rc::clone(&current);
        current = Rc::clone(&current_clone.borrow().get_dir_by_name("b").unwrap());

        add_child_file(Rc::clone(&current), 3);
        add_child_file(Rc::clone(&current), 4);

        assert_eq!(10, root.borrow().get_size());

        assert_eq!(vec![3, 7], root.borrow().get_children_sizes_under(8));
    }

    #[test]
    fn test_dir_over_size() {
        // root
        // dir a
        //    file 1
        //    file 2
        // dir b
        //    file 3
        //    file 4

        let root = Rc::new(RefCell::new(TreeNode::new()));

        let mut node_a = TreeNode::new();
        node_a.name("a");
        let child_a = Rc::new(RefCell::new(node_a));
        child_a.borrow_mut().parent = Some(Rc::clone(&root));
        add_child_dir(Rc::clone(&root), Rc::clone(&child_a));

        let mut node_b = TreeNode::new();
        node_b.name("b");
        let child_b = Rc::new(RefCell::new(node_b));
        child_b.borrow_mut().parent = Some(Rc::clone(&root));
        add_child_dir(Rc::clone(&root), child_b);

        // add files to a
        let mut current = Rc::clone(&child_a);
        add_child_file(Rc::clone(&current), 1);
        add_child_file(Rc::clone(&current), 2);

        let current_clone = Rc::clone(&current);
        current = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());

        let current_clone = Rc::clone(&current);
        current = Rc::clone(&current_clone.borrow().get_dir_by_name("b").unwrap());

        add_child_file(Rc::clone(&current), 3);
        add_child_file(Rc::clone(&current), 4);

        assert_eq!(10, root.borrow().get_size());

        assert_eq!(vec![3, 7], root.borrow().get_children_sizes_over(2));
    }
}
