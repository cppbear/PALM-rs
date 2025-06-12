// Answer 0

#[test]
fn test_locations_empty() {
    struct TestExec {
        ro: Arc<ExecReadOnly>,
        cache: CachedThreadLocal<ProgramCache>,
    }

    struct TestRegex(Exec);

    impl TestRegex {
        fn new() -> Self {
            TestRegex(Exec {
                ro: Arc::new(ExecReadOnly {}),
                cache: CachedThreadLocal::new(),
            })
        }

        fn locations(&self) -> Locations {
            self.0.searcher_str().locations()
        }
    }

    let regex = TestRegex::new();
    let locations = regex.locations();
    assert_eq!(locations.0.len(), 0);
}

#[test]
fn test_locations_non_empty() {
    struct TestExec {
        ro: Arc<ExecReadOnly>,
        cache: CachedThreadLocal<ProgramCache>,
    }

    struct TestRegex(Exec);

    impl TestRegex {
        fn new() -> Self {
            TestRegex(Exec {
                ro: Arc::new(ExecReadOnly {}),
                cache: CachedThreadLocal::new(),
            })
        }

        fn locations(&self) -> Locations {
            self.0.searcher_str().locations()
        }
    }

    let regex = TestRegex::new();
    // Assume we have a method like `add_capture` for the sake of this test
    regex.0.ro.add_capture("test".to_string()); // Simulate capturing some locations
    let locations = regex.locations();
    assert!(locations.0.len() > 0);
}

