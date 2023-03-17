use std::cmp::{max, Ordering};
use std::collections::HashMap;
use std::collections::btree_map::BTreeMap;
use std::ops::Add;
use std::ptr::NonNull;
use regex::Regex;


const COLON: &'static str = ":";

/// 平衡因子
const FACTOR: i64 = 2;

struct PathTree {
    root: Option<Box<PathNode>>,
}


impl PathTree {

    fn add_node(&mut self, mut other: PathNode) {
        if let Some(head) = &mut self.root {
            head.add_node(other);
        } else {
            self.root = Some(Box::new(other));
        }

        self.balance();
    }

    fn balance(&mut self) {
        let left_height = self.left_node_height();
        let right_height = self.right_node_height();
        let diff = left_height - right_height;

        if diff.abs() > FACTOR {
            if left_height > right_height {
                // todo!() 右旋转
            } else {
                // todo!() 左旋转
            }
        }
    }


    fn height(&self) -> i64 {
        let left_height = self.left_node_height();
        println!("left height: {}", left_height);
        let right_height = self.right_node_height();
        println!("right height: {}", right_height);
        max(left_height, right_height)
    }

    fn left_node_height(&self) -> i64 {
        match &self.root {
            Some(r) => {
                if let Some(node) = &r.left {
                    node.height()
                } else {
                    0
                }
            },
            None => 0,
        }
    }

    fn right_node_height(&self) -> i64 {
        match &self.root {
            Some(r) => {
                if let Some(node) = &r.right {
                    node.height()
                } else {
                    0
                }
            },
            None => 0,
        }
    }

    fn println_node(&self) {
        match &self.root {
            Some(head) => unsafe {
                head.println_path()
            },
            None => println!("path tree is empty")
        }
    }
}

impl Default for PathTree {
    fn default() -> Self {
        PathTree {
            root: None
        }
    }
}

struct PathNode {
    path: &'static str,
    regex: Regex,
    left: Option<Box<PathNode>>,
    right: Option<Box<PathNode>>,
    level: usize,
    tags:Vec<String>,
    len: usize,
}


impl PathNode {

    // route_path: /getById/:id
    fn new(path: &'static str) -> PathNode {
        let regex_vec = path.split("/")
            .filter(|s| !s.is_empty())
            .map(|s| {
                if s.starts_with(COLON) {
                    let tag_name = s.replace(COLON, "");
                    let capture = format!("(/)(?P<{}>((\\S)+))", tag_name);
                    (capture, Some(tag_name))
                } else {
                    let fix_str = format!("(/{})", s);
                    (fix_str, None)
                }
            }).collect::<Vec<(String, Option<String>)>>();

        let level = regex_vec.len();

        let tags = regex_vec.iter()
            .filter(|(_, o)| o.is_some())
            .map(|(_, o)| o.clone().unwrap())
            .collect::<Vec<String>>();

        let regex_str = regex_vec.iter().map(|(s, _)| s.as_str())
            .collect::<Vec<&str>>()
            .join("");
        let regex_str = format!("^{}", regex_str);
        let regex = Regex::new(regex_str.as_str()).unwrap();

        PathNode {
            path,
            regex,
            left: None,
            right: None,
            level,
            tags,
            len: 0
        }
    }

    fn height(&self) -> i64 {
        let lh = self.left_height();
        let rh = self.right_height();
        max(lh, rh)
    }

    fn left_height(&self) -> i64 {
        match &self.left {
            Some(ln) => ln.height() + 1,
            None => 0,
        }
    }

    fn right_height(&self) -> i64 {
        match &self.right {
            Some(ln) => ln.height() + 1,
            None => 0,
        }
    }

    // for test
    fn println_path(&self) {
        match &self.left {
            Some(n) => {
                println!("{} 的左边是 {}", self.path, n.path);
                n.println_path()
            },
             _ => {},
        }
        // println!("size: {}, path: {}", self.level, self.path);
        match &self.right {
            Some(n) => {
                println!("{} 的由边是 {}", self.path, n.path);
                n.println_path()
            },
            _ => {},
        }
    }

    fn into_val(self: Box<Self>) -> PathNode {
        Box::into_inner(self)
    }

    fn add_node(&mut self, mut other: PathNode) {
        if self < &mut other {
            match &mut self.left {
                Some(n) => {
                    n.add_node(other);
                },
                None => {
                    self.left = Some(Box::new(other));
                },
            }
        } else if self > &mut other {
            match &mut self.right {
                Some(n) =>  unsafe {
                    n.add_node(other);
                },
                None => {
                    self.right = Some(Box::new(other));
                },
            }
        }
        else if self == &mut other {
            // do nothing
        }
    }

    fn pattern(&self, path: &str, deep: usize) -> bool {
        self.level == deep && self.regex.is_match(path)
    }

    fn level(&self,) -> usize {
        self.level
    }

    pub(crate) fn pattern_and_extract(&self, path: &str, deep: usize)  -> Option<HashMap<String, String>> {
        if self.level == deep && self.regex.is_match(path) {
            self.extract_param(path)
        } else if let Some(l_node) = &self.left {
            l_node.pattern_and_extract(path, deep)
        } else if let Some(r_node) = &self.right {
            r_node.pattern_and_extract(path, deep)
        } else {
            None
        }
    }

    fn extract_param(&self, path: &str) -> Option<HashMap<String, String>> {
        let mut params:HashMap<String, String> = HashMap::new();
        let caps = &self.regex.captures(path).unwrap();
        for tag_name in &self.tags {
            let o = &caps[tag_name.as_str()];
            params.insert(tag_name.clone(), o.to_string());
        }
        if params.is_empty() {
            None
        } else {
            Some(params)
        }
    }

}

impl Eq for PathNode {}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.path.cmp(&other.path)
    }
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.path.cmp(&other.path))
    }

    fn lt(&self, other: &Self) -> bool {
        self.path < other.path
    }

    fn le(&self, other: &Self) -> bool {
        self.path <= other.path
    }

    fn gt(&self, other: &Self) -> bool {
        self.path > other.path
    }

    fn ge(&self, other: &Self) -> bool {
        self.path >= other.path
    }
}

impl PartialEq for PathNode {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}


#[test]
fn cmp_tree() {

    let mut tree = PathTree::default();
    let node1 = PathNode::new("/getById/:id");
    let node2 = PathNode::new("/getByName/:name");
    let node3 = PathNode::new("/:age/:name/:id");
    let node4 = PathNode::new("/getByName/:name");
    let node5 = PathNode::new("/asdfsdf/:id");
    let node6 = PathNode::new("/acdfsdf/:id");
    let node7 = PathNode::new("/aadfsdf/:id");

    tree.add_node(node1);
    tree.add_node(node2);
    tree.add_node(node3);
    tree.add_node(node4);
    tree.add_node(node5);
    tree.add_node(node6);
    tree.add_node(node7);

    tree.println_node();

    let height: &i64 = &tree.height();
    println!("height: {}", height);
}


#[test]
fn match_test() {
    let node1 = PathNode::new("/getById/:id");
    let map1 = node1.pattern_and_extract("/getById/100", 2).unwrap();
    assert_eq!(map1.get("id"), Some(&"100".to_string()));

    let node2 = PathNode::new("/:name/query/:id");
    let map2 = node2.pattern_and_extract("/张三/query/100", 3).unwrap();
    assert_eq!(map2.get("id"), Some(&"100".to_string()));
    assert_eq!(map2.get("name"), Some(&"张三".to_string()));

    let node3 = PathNode::new("/getById/:id");
    let map3 = node3.pattern_and_extract("/getByName/100", 2);
    assert!(map3.is_none());
}
