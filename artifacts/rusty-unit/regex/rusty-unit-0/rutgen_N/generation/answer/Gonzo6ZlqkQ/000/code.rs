// Answer 0

#[derive(Debug)]
struct CaptureNames<'a>(&'a [Option<&'a str>]);

struct MyStruct {
    captures: Vec<Option<String>>,
}

impl MyStruct {
    fn capture_names(&self) -> &[Option<String>] {
        &self.captures
    }
}

impl MyStruct {
    pub fn capture_names_iterator(&self) -> CaptureNames {
        CaptureNames(self.capture_names().iter().map(|s| s.as_deref()).collect::<Vec<_>>().as_slice())
    }
}

#[test]
fn test_capture_names_empty() {
    let my_struct = MyStruct { captures: Vec::new() };
    let names: CaptureNames = my_struct.capture_names_iterator();
    assert_eq!(names.0.collect::<Vec<_>>(), vec![]);
}

#[test]
fn test_capture_names_single() {
    let my_struct = MyStruct { captures: vec![Some("name1".to_string())] };
    let names: CaptureNames = my_struct.capture_names_iterator();
    assert_eq!(names.0.collect::<Vec<_>>(), vec![Some("name1")]);
}

#[test]
fn test_capture_names_multiple() {
    let my_struct = MyStruct { captures: vec![Some("name1".to_string()), None, Some("name2".to_string())] };
    let names: CaptureNames = my_struct.capture_names_iterator();
    assert_eq!(names.0.collect::<Vec<_>>(), vec![Some("name1"), None, Some("name2")]);
}

