// Answer 0

#[derive(Debug)]
struct TestStruct {
    swap_greed: Option<bool>,
}

impl TestStruct {
    fn new(swap_greed: Option<bool>) -> Self {
        TestStruct { swap_greed }
    }
    
    fn swap_greed(&self) -> bool {
        self.swap_greed.unwrap_or(false)
    }
}

#[test]
fn test_swap_greed_none() {
    let test_struct = TestStruct::new(None);
    assert_eq!(test_struct.swap_greed(), false);
}

#[test]
fn test_swap_greed_some_false() {
    let test_struct = TestStruct::new(Some(false));
    assert_eq!(test_struct.swap_greed(), false);
}

#[test]
fn test_swap_greed_some_true() {
    let test_struct = TestStruct::new(Some(true));
    assert_eq!(test_struct.swap_greed(), true);
}

