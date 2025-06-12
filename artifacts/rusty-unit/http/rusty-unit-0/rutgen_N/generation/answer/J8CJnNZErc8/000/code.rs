// Answer 0

#[test]
fn test_borrow() {
    struct MyStruct {
        value: String,
    }

    impl MyStruct {
        fn as_str(&self) -> &str {
            &self.value
        }

        fn borrow(&self) -> &str {
            self.as_str()
        }
    }

    let my_struct = MyStruct {
        value: String::from("Hello, world!"),
    };

    let borrowed_str = my_struct.borrow();
    assert_eq!(borrowed_str, "Hello, world!");
}

