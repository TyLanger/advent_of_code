use std::{cell::RefCell, fs, rc::Rc};

fn main() {
    // remember to change the input
    let file = fs::read_to_string("./inputs/day_07_input.txt").unwrap();
    println!("{}", part_1(&file));
    // println!("{}", part_2(&file));
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
    //

    // order
    // start at root
    // insert each file
    // insert each node (without their files)
    // go into one of the nodes
    // update its files
    // go back
    // go into the other node
    // update its files

    let root = Rc::new(RefCell::new(TreeNode::new()));
    let mut current = Rc::clone(&root);

    let lines = input.lines().skip(1);
    for line in lines {
        if line.contains("$ cd") {
            // name
            // ..

            let arg = line.split_once("cd ").unwrap().1;
            // println!("arg: {:?}", arg);

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
            // println!("file value: {}", value);
            // current.borrow_mut().insert_file(size.parse().unwrap());
            add_child_file(Rc::clone(&current), value);
        }
    }

    // println!("root to_string: {:?}", root.borrow().to_string());
    let v = root.borrow().get_children_sizes_under(100000);
    let mut size = 0;
    // println!("v len: {}", v.len());
    for item in v {
        size += item;
    }
    size
}

fn add_child_file(current: Rc<RefCell<TreeNode>>, value: u32) {
    // let child = Rc::new(RefCell::new(TreeNode::new()));
    // current.borrow_mut().children.push(Rc::clone(&child));

    // let mut mut_child = child.borrow_mut();
    // mut_child.parent = Some(Rc::clone(&current));
    // mut_child.files.push(value);

    current.borrow_mut().files.push(value);
}

fn add_child_dir(current: Rc<RefCell<TreeNode>>, child: Rc<RefCell<TreeNode>>) {
    current.borrow_mut().children.push(Rc::clone(&child));
}

// maybe this is what I need
// https://applied-math-coding.medium.com/a-tree-structure-implemented-in-rust-8344783abd75
struct TreeNode {
    files: Vec<u32>,
    directories: Vec<TreeNode>,
    name: String,
    children: Vec<Rc<RefCell<TreeNode>>>,
    parent: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new() -> Self {
        TreeNode {
            files: vec![],
            directories: vec![],
            name: "".to_string(),
            children: vec![],
            parent: None,
        }
    }

    fn insert_file(&mut self, file_size: u32) {
        self.files.push(file_size);
    }

    fn insert_node(&mut self, node: TreeNode) {
        self.directories.push(node);
    }

    fn add_child(&mut self, new_node: Rc<RefCell<TreeNode>>) {
        self.children.push(new_node);
    }

    fn name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn get_size(&self) -> u32 {
        let mut self_count = 0;
        for f in &self.files {
            self_count += f;
        }

        let mut dir_count = 0;

        for d in &self.directories {
            dir_count += d.get_size();
        }

        let mut child_count = 0;
        for c in &self.children {
            child_count += c.borrow().get_size();
        }

        // cache the size
        self_count + dir_count + child_count
    }

    fn get_sub_dir_sizes_under(&self, under: u32) -> Vec<u32> {
        let mut v = vec![];

        for d in &self.directories {
            let size = d.get_size();
            if size < under {
                v.push(size);
            }

            let v_children = d.get_sub_dir_sizes_under(under);

            for item in v_children {
                v.push(item);
            }
        }

        v
    }

    fn get_children_sizes_under(&self, under: u32) -> Vec<u32> {
        let mut v = vec![];
        // println!("children len {}", self.children.len());

        // if self.children.is_empty() {
        //     let size = self.get_size();
        //     if size < under {
        //         v.push(size);
        //     }
        // }

        for c in &self.children {
            let size: u32 = c.borrow().get_size();
            if size < under {
                // println!("Adding size {}", size);
                v.push(size);
            }

            let v_children = c.borrow().get_children_sizes_under(under);

            for item in v_children {
                // println!("Adding item {}", item);
                v.push(item);
            }
        }

        v
    }

    fn get_node(&self, name: &str) -> Option<&TreeNode> {
        for d in &self.directories {
            if d.name == name {
                return Some(d);
            }
        }
        None
    }

    fn get_dir_by_name(&self, name: &str) -> Option<Rc<RefCell<TreeNode>>> {
        for c in &self.children {
            if c.borrow().name == name {
                return Some(Rc::clone(c));
            }
        }
        None
    }

    fn is_leaf(&self) -> bool {
        // self.directories.is_empty()
        self.children.is_empty() && self.directories.is_empty()
    }

    fn to_string(&self) -> String {
        let mut output = "".to_string();
        for f in &self.files {
            output = format!("{}file {}\n", output, f);
        }

        for c in &self.children {
            // c.borrow().to_string()
            output = format!("{}child: {}\n", output, c.borrow().to_string());
        }

        output
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
    // #[ignore = "not ready"]
    fn part_1_works() {
        let result = part_1(&DAY_7_BASIC_INPUT);

        assert_eq!(95437, result);
    }

    #[test]
    fn empty_tree_size_is_0() {
        let node = TreeNode::new();

        assert_eq!(0, node.get_size());
    }

    #[test]
    fn lone_node_is_leaf() {
        let node = TreeNode::new();

        assert!(node.is_leaf());
    }

    #[test]
    fn tree_node_insert_file() {
        let mut node = TreeNode::new();
        node.insert_file(123);
        node.insert_file(456);

        assert_eq!(579, node.get_size());
    }

    #[test]
    fn root_is_not_leaf() {
        let mut root = TreeNode::new();
        let leaf = TreeNode::new();
        root.insert_node(leaf);

        // let leaf = Rc::new(RefCell::new(TreeNode::new()));
        // root.add_child(leaf);
        // leaf.borrow_mut().parent = Some(Rc::new(RefCell::new(root)));

        // assert!(leaf.is_leaf());
        assert!(!root.is_leaf());
    }

    #[test]
    fn tree_with_children() {
        let mut root = TreeNode::new();
        root.insert_file(1);
        root.insert_file(2);

        let mut node = TreeNode::new();
        node.insert_file(3);

        root.insert_node(node);

        assert_eq!(6, root.get_size());
    }

    #[test]
    fn get_directories_with_size_under_x() {
        let mut root = TreeNode::new();
        root.insert_file(4);
        root.insert_file(2);

        // sub dir a
        let mut node = TreeNode::new();
        node.insert_file(1);
        node.insert_file(2);

        root.insert_node(node);

        // sub dir b
        let mut node = TreeNode::new();
        node.insert_file(4);

        root.insert_node(node);

        // fails
        let mut node = TreeNode::new();
        node.insert_file(4);
        node.insert_file(6);

        root.insert_node(node);

        let dirs = root.get_sub_dir_sizes_under(10);

        assert_eq!(vec![3, 4], dirs);
    }

    #[test]
    fn get_sub_dir_inside_too_big_dir() {
        let mut root = TreeNode::new();

        let mut mid = TreeNode::new();
        mid.insert_file(11);

        let mut leaf = TreeNode::new();
        leaf.insert_file(3);

        mid.insert_node(leaf);
        root.insert_node(mid);

        let dirs = root.get_sub_dir_sizes_under(10);

        assert_eq!(vec![3], dirs);
    }

    #[test]
    fn get_dir_by_name_test() {
        let mut root = TreeNode::new();

        let mut node_1 = TreeNode::new();
        node_1.name("abc");
        node_1.insert_file(3);
        root.insert_node(node_1);

        let mut node_2 = TreeNode::new();
        node_2.name("def");
        node_2.insert_file(4);
        root.insert_node(node_2);

        let size = root.get_node("abc").unwrap().get_size();

        assert_eq!(3, size);

        let doesnt_exist = root.get_node("ghi");
        assert!(doesnt_exist.is_none());
    }

    #[test]
    fn init_tree() {
        let root = Rc::new(RefCell::new(TreeNode::new()));
        let mut current = Rc::clone(&root);

        add_child_file(current, 14);

        // let child = Rc::new(RefCell::new(TreeNode::new()));
        // current.borrow_mut().children.push(Rc::clone(&child));
        // {
        //     let mut mut_child = child.borrow_mut();
        //     mut_child.parent = Some(Rc::clone(&current));
        //     mut_child.files.push(14);
        // }

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
        let out = Rc::clone(&child);

        add_child_file(current, 3);
        current = Rc::clone(&root);
        add_child_dir(current, child);

        assert_eq!(6, root.borrow().get_size());
        assert!(!root.borrow().is_leaf());
        assert!(out.borrow().is_leaf());
    }

    #[test]
    fn add_x_children_with_files() {
        let root = Rc::new(RefCell::new(TreeNode::new()));

        let mut current = Rc::clone(&root);
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
}
