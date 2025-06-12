// Answer 0

#[test]
fn test_fmt_debug() {
    use std::fmt;

    struct MyStruct(Vec<i32>);

    impl fmt::Debug for MyStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }

    let instance = MyStruct(vec![1, 2, 3]);
    let debug_output = format!("{:?}", instance);
    assert_eq!(debug_output, "[1, 2, 3]");
}

