use std::{fs, cell::RefCell, rc::{self, Rc}};

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

    let mut root = Rc::new(RefCell::new( TreeNode::new()));
    let mut current = Rc::clone(&root);

    let lines = input.lines();
    for line in lines {
        if line.contains("$ cd") {
            // name
            // ..

            let arg = line.split_once("cd ").unwrap().1;

            if arg.contains("..") {
                current = Rc::clone(&root);
            } else {
                let node = current.borrow().get_node(arg).unwrap();
                // current = Rc::clone(node);
            }
        } else if line.contains("$ ls") {
            // next few lines will be files and dirs
        } else if line.contains("dir") {
            let name = line.split_once("dir ").unwrap().1;

            let mut node = TreeNode::new();
            node.name = name.to_string();
            current.borrow_mut().insert_node(node)
        } else {
            // 12312398 file.txt
            let size = line.split_once(' ').unwrap().0;
            current.borrow_mut().insert_file(size.parse().unwrap());
        }
    }

    let v = root.borrow().get_sub_dir_sizes_under(100000);
    let mut size = 0;
    for item in v {
        size += item;
    }
    size
}

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

        // cache the size
        self_count + dir_count
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

    fn get_node(&self, name: &str) -> Option<&TreeNode> {
        for d in &self.directories {
            if d.name == name {
                return Some(d);
            }
        }
        None
    }

    fn is_leaf(&self) -> bool {
        self.directories.is_empty()
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
    #[ignore = "not ready"]
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
    fn get_dir_by_name() {
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
}
