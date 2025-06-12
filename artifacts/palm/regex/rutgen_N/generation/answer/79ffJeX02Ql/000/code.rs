// Answer 0

#[test]
fn test_approximate_size() {
    struct Dummy;

    impl Dummy {
        pub fn approximate_size(&self) -> usize { 0 }
    }

    let dummy = Dummy;
    assert_eq!(dummy.approximate_size(), 0);
}

