// Answer 0

#[derive(Debug)]
struct MaxSizeReached {
    _priv: (),
}

impl MaxSizeReached {
    fn new() -> Self {
        MaxSizeReached { _priv: () }
    }
}

#[test]
fn test_new_max_size_reached() {
    let instance = MaxSizeReached::new();
    assert_eq!(format!("{:?}", instance), "MaxSizeReached {_priv: ()}");
}

