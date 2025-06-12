// Answer 0

#[test]
fn test_fmt() {
    use std::fmt;

    struct Lazy {
        cell: i32,
    }

    impl fmt::Debug for Lazy {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("Lazy").field("cell", &self.cell).field("init", &"..").finish()
        }
    }

    let lazy_instance = Lazy { cell: 42 };
    let result = format!("{:?}", lazy_instance);
    
    assert_eq!(result, "Lazy { cell: 42, init: \"..\" }");
}

