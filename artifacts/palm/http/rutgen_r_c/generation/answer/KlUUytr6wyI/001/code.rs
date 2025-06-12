// Answer 0

#[test]
fn test_body_mut_initialization() {
    struct MyRequest {
        body: String,
    }

    impl MyRequest {
        fn body_mut(&mut self) -> &mut String {
            &mut self.body
        }
    }

    let mut request = MyRequest {
        body: String::new(),
    };

    assert_eq!(request.body_mut().len(), 0);
    request.body_mut().push_str("hello world");
    assert_eq!(request.body_mut().len(), 11);
    assert!(!request.body().is_empty());
}

#[test]
fn test_body_mut_with_empty_body() {
    struct MyRequest {
        body: Vec<u8>,
    }

    impl MyRequest {
        fn body_mut(&mut self) -> &mut Vec<u8> {
            &mut self.body
        }
        
        fn body(&self) -> &Vec<u8> {
            &self.body
        }
    }

    let mut request = MyRequest {
        body: Vec::new(),
    };

    assert_eq!(request.body_mut().len(), 0);
    request.body_mut().push(1);
    assert_eq!(request.body().len(), 1);
}

#[test]
fn test_body_mut_modification() {
    struct MyRequest {
        body: i32,
    }

    impl MyRequest {
        fn body_mut(&mut self) -> &mut i32 {
            &mut self.body
        }
        
        fn body(&self) -> &i32 {
            &self.body
        }
    }

    let mut request = MyRequest { body: 5 };
    
    assert_eq!(*request.body_mut(), 5);
    *request.body_mut() += 5;
    assert_eq!(*request.body(), 10);
}

#[test]
#[should_panic]
fn test_body_mut_panic_on_accessing_uninitialized_body() {
    struct MyRequest {
        body: Option<String>,
    }

    impl MyRequest {
        fn body_mut(&mut self) -> &mut Option<String> {
            &mut self.body
        }

        fn body(&self) -> &Option<String> {
            &self.body
        }
    }
    
    let mut request = MyRequest { body: None };
    let _body = request.body_mut().as_mut().unwrap(); // This will panic
}

