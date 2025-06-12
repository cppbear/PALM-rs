// Answer 0

#[derive(Debug)]
struct RegexWrapper(&'static str);

impl RegexWrapper {
    fn replace_append(&mut self, _: &str, dst: &mut String) {
        dst.push_str(self.0);
    }
}

#[test]
fn test_replace_append() {
    let mut regex_wrapper = RegexWrapper("replacement");
    let mut destination = String::new();

    regex_wrapper.replace_append("", &mut destination);
    
    assert_eq!(destination, "replacement");
}

#[test]
fn test_replace_append_with_non_empty_dst() {
    let mut regex_wrapper = RegexWrapper("replacement");
    let mut destination = String::from("initial");

    regex_wrapper.replace_append("", &mut destination);
    
    assert_eq!(destination, "initialreplacement");
}

#[test]
fn test_replace_append_empty_string() {
    let mut regex_wrapper = RegexWrapper("");
    let mut destination = String::new();

    regex_wrapper.replace_append("", &mut destination);
    
    assert_eq!(destination, "");
}

#[test]
fn test_replace_append_edge_case() {
    let mut regex_wrapper = RegexWrapper("edge");
    let mut destination = String::new();

    regex_wrapper.replace_append("", &mut destination);
    regex_wrapper.replace_append("", &mut destination);
    
    assert_eq!(destination, "edgeedge");
}

