// Answer 0

#[derive(Debug)]
struct MyStruct {
    tag: String,
    content: String,
}

impl MyStruct {
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{:?} or {:?}", self.tag, self.content)
    }
}

#[test]
fn test_expecting() {
    let my_struct = MyStruct {
        tag: String::from("example_tag"),
        content: String::from("example_content"),
    };

    let mut output = String::new();
    {
        let mut formatter = std::fmt::Formatter::new(&mut output);
        my_struct.expecting(&mut formatter).unwrap();
    }

    assert_eq!(output, "\"example_tag\" or \"example_content\"");
}

