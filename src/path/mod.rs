use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Add;
use std::ptr::NonNull;
use regex::Regex;


const COLON: &'static str = ":";

struct PathTree {
    root: Option<PathNode>,
}


impl PathTree {

    fn add_node(&mut self, other: PathNode) {
        if let Some(head) = &mut self.root {
            head.add_node(other);
        } else {
            self.root = Some(other);
        }
    }
}

struct PathNode {
    path: String,
    regex: Regex,
    left: Option<NonNull<PathNode>>,
    right: Option<NonNull<PathNode>>,
    level: usize,
    tags:Vec<String>
}


impl PathNode {

    // route_path: /getById/:id
    fn new(path: &str) -> PathNode {
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
            path: path.to_owned(),
            regex,
            left: None,
            right: None,
            level,
            tags
        }
    }

    fn add_node(&mut self, mut other: PathNode) {
        if self < &mut other {

        } else if self > &mut other {

        } else if self == &mut other {
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
        } else if let Some(l_node) = self.left {
            unsafe {
                l_node.as_ref().pattern_and_extract(path, deep)
            }
        } else if let Some(r_node) = self.left {
            unsafe {
                r_node.as_ref().pattern_and_extract(path, deep)
            }
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
        self.level.cmp(&other.level)
    }
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.path.cmp(&other.path))
    }
}

impl PartialEq for PathNode {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self.path[..], &other.path[..])
    }
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
