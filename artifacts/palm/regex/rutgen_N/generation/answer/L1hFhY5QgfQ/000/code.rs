// Answer 0

#[test]
fn test_searcher() {
    struct TestStruct {
        ro: String,
        cache: RefCell<HashMap<String, Box<RefCell<ProgramCacheInner>>>>,
    }

    impl TestStruct {
        fn new(ro: String) -> Self {
            TestStruct {
                ro,
                cache: RefCell::new(HashMap::new()),
            }
        }

        pub fn searcher(&self) -> ExecNoSync {
            let create = || Box::new(RefCell::new(ProgramCacheInner::new(&self.ro)));
            ExecNoSync {
                ro: &self.ro,
                cache: self.cache.borrow_mut().entry(self.ro.clone()).or_insert_with(create),
            }
        }
    }

    struct ExecNoSync<'a> {
        ro: &'a String,
        cache: Box<RefCell<ProgramCacheInner>>,
    }

    struct ProgramCacheInner<'a> {
        ro: &'a String,
    }

    impl<'a> ProgramCacheInner<'a> {
        fn new(ro: &'a String) -> Self {
            ProgramCacheInner { ro }
        }
    }

    let test_data = "sample_data".to_string();
    let test_struct = TestStruct::new(test_data.clone());

    let searcher_result = test_struct.searcher();

    assert_eq!(searcher_result.ro, &test_data);
    assert!(searcher_result.cache.borrow().ro == &test_data);
}

