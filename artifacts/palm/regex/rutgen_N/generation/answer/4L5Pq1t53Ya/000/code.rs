// Answer 0

#[derive(Debug)]
struct DummyCaptureNames<'a>(&'a [&'a str]);

struct RegexWrapper<'a>(DummyCaptureNames<'a>);

impl<'a> RegexWrapper<'a> {
    pub fn capture_names(&self) -> DummyCaptureNames<'a> {
        self.0
    }
}

#[test]
fn test_capture_names() {
    let names: Vec<&str> = vec!["name1", "name2", "name3"];
    let regex = RegexWrapper(DummyCaptureNames(&names));
    
    let capture_names_iter = regex.capture_names();
    
    let result: Vec<&str> = capture_names_iter.0.iter().cloned().collect();
    
    assert_eq!(result, names);
}

#[test]
fn test_capture_names_empty() {
    let names: Vec<&str> = vec![];
    let regex = RegexWrapper(DummyCaptureNames(&names));
    
    let capture_names_iter = regex.capture_names();
    
    let result: Vec<&str> = capture_names_iter.0.iter().cloned().collect();
    
    assert_eq!(result, names);
}

