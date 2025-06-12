// Answer 0

#[test]
fn test_fmt_with_empty_iterator() {
    struct TestStruct;

    impl TestStruct {
        fn iter(&self) -> std::slice::Iter<'static, i32> {
            [].iter() 
        }
    }

    let test_struct = TestStruct;
    let output = format!("{:?}", test_struct);
    assert_eq!(output, "[]");
}

#[test]
fn test_fmt_with_single_element() {
    struct TestStruct {
        elements: Vec<i32>
    }

    impl TestStruct {
        fn iter(&self) -> std::slice::Iter<'_, i32> {
            self.elements.iter() 
        }
    }

    let test_struct = TestStruct { elements: vec![42] };
    let output = format!("{:?}", test_struct);
    assert_eq!(output, "[42]");
}

#[test]
fn test_fmt_with_multiple_elements() {
    struct TestStruct {
        elements: Vec<i32>
    }

    impl TestStruct {
        fn iter(&self) -> std::slice::Iter<'_, i32> {
            self.elements.iter() 
        }
    }

    let test_struct = TestStruct { elements: vec![1, 2, 3] };
    let output = format!("{:?}", test_struct);
    assert_eq!(output, "[1, 2, 3]");
}

#[test]
#[should_panic]
fn test_fmt_with_panic() {
    struct PanicStruct;

    impl PanicStruct {
        fn iter(&self) -> std::slice::Iter<'static, i32> {
            panic!("Intentional Panic")
        }
    }

    let panic_struct = PanicStruct;
    let _output = format!("{:?}", panic_struct);
}

