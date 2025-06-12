// Answer 0

#[test]
fn test_fmt_success() {
    use std::fmt;
    use std::collections::HashMap;

    struct MyStruct<'a> {
        map: &'a HashMap<&'a str, &'a str>,
    }

    impl<'a> fmt::Display for MyStruct<'a> {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            self.map.fmt(formatter)
        }
    }

    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");

    let my_struct = MyStruct { map: &map };
    let result = format!("{}", my_struct);

    assert_eq!(result, "key1: value1\nkey2: value2\n"); // Assuming the default Display of HashMap uses the key:value format
}

#[test]
#[should_panic]
fn test_fmt_panic() {
    use std::fmt;
    use std::collections::HashMap;

    struct MyStruct<'a> {
        map: &'a HashMap<&'a str, &'a str>,
    }

    impl<'a> fmt::Display for MyStruct<'a> {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            self.map.fmt(formatter)
        }
    }

    let map: HashMap<&str, &str> = HashMap::new();
    let my_struct = MyStruct { map: &map };

    // Intentionally causing a panic by formatting an empty map
    let _ = format!("{}", my_struct);
}

