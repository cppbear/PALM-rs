// Answer 0

#[test]
fn test_locations_empty_set() {
    struct SearcherMock;

    impl SearcherMock {
        fn locations(&self) -> Locations {
            // Mocked implementation returns an empty Locations
            Locations::new()
        }
    }

    struct Wrapper {
        searcher: SearcherMock,
    }

    impl Wrapper {
        fn searcher(&self) -> &SearcherMock {
            &self.searcher
        }
    }

    let wrapper = Wrapper {
        searcher: SearcherMock,
    };

    let locs = wrapper.locations();
    assert!(locs.is_empty());
}

#[test]
fn test_locations_non_empty_set() {
    struct SearcherMock;

    impl SearcherMock {
        fn locations(&self) -> Locations {
            // Mocked implementation returns a non-empty Locations
            let mut locs = Locations::new();
            locs.push(0..5); // Example range
            locs
        }
    }

    struct Wrapper {
        searcher: SearcherMock,
    }

    impl Wrapper {
        fn searcher(&self) -> &SearcherMock {
            &self.searcher
        }
    }

    let wrapper = Wrapper {
        searcher: SearcherMock,
    };

    let locs = wrapper.locations();
    assert!(!locs.is_empty());
    assert_eq!(locs.len(), 1); // Expect one location
    assert_eq!(locs[0], 0..5); // Check the specific location
}

