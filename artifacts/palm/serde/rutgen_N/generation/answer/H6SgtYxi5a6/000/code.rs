// Answer 0

#[derive(Debug)]
struct MyStruct(usize);

impl std::fmt::Display for MyStruct {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.0 == 1 {
            formatter.write_str("1 element in map")
        } else {
            write!(formatter, "{} elements in map", self.0)
        }
    }
}

#[test]
fn test_fmt_single_element() {
    let value = MyStruct(1);
    let result = format!("{}", value);
    assert_eq!(result, "1 element in map");
}

#[test]
fn test_fmt_multiple_elements() {
    let value = MyStruct(5);
    let result = format!("{}", value);
    assert_eq!(result, "5 elements in map");
}

