// Answer 0

#[derive(Debug)]
struct MyStruct(Option<i32>, Option<i32>);

impl MyStruct {
    fn size_hint(&self) -> Option<usize> {
        if self.0.is_some() {
            Some(2)
        } else if self.1.is_some() {
            Some(1)
        } else {
            Some(0)
        }
    }
}

#[test]
fn test_size_hint_both_options_some() {
    let my_struct = MyStruct(Some(1), Some(2));
    assert_eq!(my_struct.size_hint(), Some(2));
}

#[test]
fn test_size_hint_first_option_some() {
    let my_struct = MyStruct(Some(1), None);
    assert_eq!(my_struct.size_hint(), Some(2));
}

#[test]
fn test_size_hint_second_option_some() {
    let my_struct = MyStruct(None, Some(2));
    assert_eq!(my_struct.size_hint(), Some(1));
}

#[test]
fn test_size_hint_both_options_none() {
    let my_struct = MyStruct(None, None);
    assert_eq!(my_struct.size_hint(), Some(0));
}

