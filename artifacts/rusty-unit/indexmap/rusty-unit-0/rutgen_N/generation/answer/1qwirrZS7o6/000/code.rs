// Answer 0

#[test]
fn test_into_entries() {
    struct DummyEntry;
    
    struct DummyCore {
        entries: Vec<DummyEntry>,
    }

    impl DummyCore {
        fn into_entries(self) -> Vec<DummyEntry> {
            self.entries
        }
    }

    struct DummyMap {
        core: DummyCore,
    }

    impl DummyMap {
        fn new(entries: Vec<DummyEntry>) -> Self {
            Self { core: DummyCore { entries } }
        }

        fn into_entries(self) -> Vec<DummyEntry> {
            self.core.into_entries()
        }
    }

    let entries = vec![DummyEntry, DummyEntry];
    let map = DummyMap::new(entries.clone());
    
    let result = map.into_entries();
    assert_eq!(result.len(), entries.len());
}

