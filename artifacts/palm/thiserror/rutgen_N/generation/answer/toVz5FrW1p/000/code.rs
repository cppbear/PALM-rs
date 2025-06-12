// Answer 0

#[derive(Debug)]
struct MyStruct;

impl MyStruct {
    fn display(&self) -> &str {
        "Display Output"
    }

    fn as_display<'a>(&'a self) -> &'a str {
        self.display()
    }
}

#[test]
fn test_as_display() {
    let my_instance = MyStruct;
    let output = my_instance.as_display();
    assert_eq!(output, "Display Output");
}

#[test]
fn test_as_display_non_empty() {
    let my_instance = MyStruct;
    let output = my_instance.as_display();
    assert!(!output.is_empty());
}

