// Answer 0

#[derive(Debug)]
struct MyStruct(usize);

impl MyStruct {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.0 == 1 {
            formatter.write_str("1 element in sequence")
        } else {
            write!(formatter, "{} elements in sequence", self.0)
        }
    }
}

#[test]
fn test_fmt_one_element() {
    let my_struct = MyStruct(1);
    let mut output = String::new();
    let result = my_struct.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "1 element in sequence");
}

#[test]
fn test_fmt_multiple_elements() {
    let my_struct = MyStruct(5);
    let mut output = String::new();
    let result = my_struct.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "5 elements in sequence");
}

#[test]
fn test_fmt_zero_elements() {
    let my_struct = MyStruct(0);
    let mut output = String::new();
    let result = my_struct.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "0 elements in sequence");
}

