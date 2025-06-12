// Answer 0

#[test]
fn test_end_with_elements() {
    struct TestStruct {
        elements: Vec<i32>,
    }

    impl TestStruct {
        fn end(self) -> Result<Content, &'static str> {
            Ok(Content::Tuple(self.elements))
        }
    }

    struct Content {
        elements: Vec<i32>,
    }

    impl Content {
        fn Tuple(elements: Vec<i32>) -> Content {
            Content { elements }
        }
    }

    let elements = vec![1, 2, 3];
    let test_struct = TestStruct { elements };
    
    let result = test_struct.end();
    match result {
        Ok(content) => {
            assert_eq!(content.elements.len(), 3);
            assert_eq!(content.elements, vec![1, 2, 3]);
        },
        Err(_) => panic!("Expected Ok but got Err"),
    }
}

#[test]
fn test_end_with_empty_elements() {
    struct TestStruct {
        elements: Vec<i32>,
    }

    impl TestStruct {
        fn end(self) -> Result<Content, &'static str> {
            Ok(Content::Tuple(self.elements))
        }
    }

    struct Content {
        elements: Vec<i32>,
    }

    impl Content {
        fn Tuple(elements: Vec<i32>) -> Content {
            Content { elements }
        }
    }

    let elements: Vec<i32> = vec![];
    let test_struct = TestStruct { elements };
    
    let result = test_struct.end();
    match result {
        Ok(content) => {
            assert_eq!(content.elements.len(), 0);
            assert_eq!(content.elements, vec![]);
        },
        Err(_) => panic!("Expected Ok but got Err"),
    }
}

