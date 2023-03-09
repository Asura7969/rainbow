use std::collections::HashMap;
use std::ops::Add;
use std::ptr::NonNull;
use regex::Regex;


const COLON: &'static str = ":";

struct PathTreeMap {

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

    fn pattern(&self, path: &str, deep: usize) -> bool {
        self.level == deep && self.regex.is_match(path)
    }

    fn level(&self,) -> usize {
        self.level
    }

    fn extract_param(&self, path: &str) -> HashMap<String, String> {
        let mut params:HashMap<String, String> = HashMap::new();
        let caps = &self.regex.captures(path).unwrap();
        for tag_name in &self.tags {
            let o = &caps[tag_name.as_str()];
            params.insert(tag_name.clone(), o.to_string());
        }

        params
    }

}

#[test]
fn match_test() {
    let node1 = PathNode::new("/getById/:id");
    let map1 = &node1.extract_param("/getById/100");
    assert_eq!(map1.get("id"), Some(&"100".to_string()));


    let node2 = PathNode::new("/:name/query/:id");
    let map2 = &node2.extract_param("/张三/query/100");
    assert_eq!(map2.get("id"), Some(&"100".to_string()));
    assert_eq!(map2.get("name"), Some(&"张三".to_string()));
}
