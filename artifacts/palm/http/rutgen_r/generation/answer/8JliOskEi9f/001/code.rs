// Answer 0

#[test]
fn test_index_mut_valid_index() {
    struct Link {
        links: usize,
    }

    struct Container(Vec<Link>);

    impl Container {
        fn index_mut(&mut self, idx: usize) -> &mut usize {
            unsafe { &mut (*self.0)[idx].links }
        }
    }

    let mut container = Container(vec![Link { links: 0 }, Link { links: 1 }]);
    let idx = 0;

    let link = container.index_mut(idx);
    *link = 42;
    assert_eq!(container.0[idx].links, 42);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_index_mut_out_of_bounds() {
    struct Link {
        links: usize,
    }

    struct Container(Vec<Link>);

    impl Container {
        fn index_mut(&mut self, idx: usize) -> &mut usize {
            unsafe { &mut (*self.0)[idx].links }
        }
    }

    let mut container = Container(vec![Link { links: 0 }]);
    let idx = 1; // Out of bounds index

    let _link = container.index_mut(idx);
}

#[test]
fn test_index_mut_boundary() {
    struct Link {
        links: usize,
    }

    struct Container(Vec<Link>);

    impl Container {
        fn index_mut(&mut self, idx: usize) -> &mut usize {
            unsafe { &mut (*self.0)[idx].links }
        }
    }

    let mut container = Container(vec![Link { links: 0 }, Link { links: 1 }]);
    let idx = 1;

    let link = container.index_mut(idx);
    *link = 99;
    assert_eq!(container.0[idx].links, 99);
}

