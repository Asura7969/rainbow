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
}


impl PathNode {

    // route_path: /getById/:id
    fn new(path: &str) -> PathNode {

        let regex_str = path.split("/")
            .filter(|s| !s.is_empty())
            .map(|s| {
                if s.starts_with(COLON) {
                    "(/*)".to_string()
                } else {
                    let es = format!("(/{})", s);
                    es
                }
            }).collect::<Vec<String>>()
            .join("");
        // let enumerate = path.split("/").enumerate();
        // for(_index, s) in enumerate {
        //     let er = if s.starts_with(COLON) {
        //         "(/:*)"
        //     } else {
        //         "(/*)"
        //     };
        // }
        let regex_str = format!("^{}", regex_str);
        println!("{}", regex_str);
        let regex = Regex::new(regex_str.as_str()).unwrap();

        PathNode {
            path: path.to_owned(),
            regex,
            left: None,
            right: None,
        }
    }

    fn pattern(&self, path: &str) -> bool {
        self.regex.is_match(path)
    }
}

#[test]
fn match_test() {
    let node = PathNode::new("/getById/:id");
    let route_path1 = "/getById/1";
    let route_path2 = "/getByName/1";
    let route_path3 = "/aa/getById/1";
    assert!(&node.pattern(route_path1));
    assert!(!&node.pattern(route_path2));
    assert!(!&node.pattern(route_path3));
}
