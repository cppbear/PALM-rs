// Answer 0

#[test]
fn test_last_non_empty() {
    struct DummyMutableValues {
        entries: Vec<Bucket<u32, ()>>,
    }
    
    impl private::Sealed for DummyMutableValues {}

    impl MutableValues for DummyMutableValues {
        type Value = ();
        
        fn get_full_mut2<Q>(&mut self, _: &Q) -> Option<(usize, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Value>,
        {
            None
        }
        
        fn get_index_mut2(&mut self, index: usize) -> Option<&mut Self::Value> {
            self.entries.get_mut(index).map(|_| &mut ())
        }

        fn retain2<F>(&mut self, _: F) where F: FnMut(&mut Self::Value) -> bool {}
    }

    let mut dummy_set = DummyMutableValues {
        entries: vec![
            Bucket { hash: 1.into(), key: 1, value: () },
            Bucket { hash: 2.into(), key: 2, value: () },
            Bucket { hash: 3.into(), key: 3, value: () },
        ],
    };

    assert_eq!(dummy_set.last(), Some(&3));
}

#[test]
fn test_last_empty() {
    struct DummyMutableValues {
        entries: Vec<Bucket<u32, ()>>,
    }
    
    impl private::Sealed for DummyMutableValues {}

    impl MutableValues for DummyMutableValues {
        type Value = ();
        
        fn get_full_mut2<Q>(&mut self, _: &Q) -> Option<(usize, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Value>,
        {
            None
        }
        
        fn get_index_mut2(&mut self, index: usize) -> Option<&mut Self::Value> {
            self.entries.get_mut(index).map(|_| &mut ())
        }

        fn retain2<F>(&mut self, _: F) where F: FnMut(&mut Self::Value) -> bool {}
    }

    let mut dummy_set = DummyMutableValues { entries: vec![] };

    assert_eq!(dummy_set.last(), None);
}

