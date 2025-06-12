// Answer 0

#[test]
fn test_builder_new() {
    struct Builder {
        method: Option<String>,
        body: Option<()>,
    }

    impl Default for Builder {
        fn default() -> Self {
            Builder { method: None, body: None }
        }
    }

    impl Builder {
        fn method(mut self, method: &str) -> Self {
            self.method = Some(method.to_string());
            self
        }

        fn body(mut self, body: ()) -> Result<Self, &'static str> {
            self.body = Some(body);
            Ok(self)
        }
    }

    let req = Builder::default()
        .method("POST")
        .body(())
        .unwrap();

    assert_eq!(req.method, Some("POST".to_string()));
    assert!(req.body.is_some());
}

