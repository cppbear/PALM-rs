// Answer 0

#[derive(Debug)]
struct Bucket {
    refs: i32,
}

struct MyStruct {
    iter: Vec<Bucket>,
}

impl MyStruct {
    fn new() -> Self {
        MyStruct { iter: Vec::new() }
    }

    fn add_bucket(&mut self, refs: i32) {
        self.iter.push(Bucket { refs });
    }
}

impl std::fmt::Debug for MyStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let iter = self.iter.iter().map(|b| b.refs);
        f.debug_list().entries(iter).finish()
    }
}

#[test]
fn test_fmt_empty() {
    let my_struct = MyStruct::new();
    let output = format!("{:?}", my_struct);
    assert_eq!(output, "[]");
}

#[test]
fn test_fmt_single_entry() {
    let mut my_struct = MyStruct::new();
    my_struct.add_bucket(42);
    let output = format!("{:?}", my_struct);
    assert_eq!(output, "[42]");
}

#[test]
fn test_fmt_multiple_entries() {
    let mut my_struct = MyStruct::new();
    my_struct.add_bucket(1);
    my_struct.add_bucket(2);
    my_struct.add_bucket(3);
    let output = format!("{:?}", my_struct);
    assert_eq!(output, "[1, 2, 3]");
}

