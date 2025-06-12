// Answer 0

#[test]
fn fmt_debug_list_test() {
    #[derive(Clone, Debug)]
    struct MySet {
        elements: Vec<i32>,
    }

    impl MySet {
        fn new() -> Self {
            MySet { elements: Vec::new() }
        }

        fn add(&mut self, value: i32) {
            self.elements.push(value);
        }
    }

    use std::fmt;

    impl fmt::Debug for MySet {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.clone().elements).finish()
        }
    }

    let mut set = MySet::new();
    set.add(1);
    set.add(2);
    
    let expected_output = "[1, 2]";
    let output = format!("{:?}", set);

    assert_eq!(output, expected_output);
}

#[test]
fn fmt_empty_debug_list_test() {
    #[derive(Clone, Debug)]
    struct MySet {
        elements: Vec<i32>,
    }

    impl MySet {
        fn new() -> Self {
            MySet { elements: Vec::new() }
        }
    }

    use std::fmt;

    impl fmt::Debug for MySet {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.clone().elements).finish()
        }
    }

    let set = MySet::new();
    
    let expected_output = "[]";
    let output = format!("{:?}", set);

    assert_eq!(output, expected_output);
}

