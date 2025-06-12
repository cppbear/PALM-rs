// Answer 0

#[derive(Debug)]
struct MockMap {
    should_return_error: bool,
}

impl MockMap {
    fn new(should_return_error: bool) -> Self {
        MockMap { should_return_error }
    }

    fn next_value(&mut self) -> Result<(), &'static str> {
        if self.should_return_error {
            Err("error")
        } else {
            Ok(())
        }
    }
}

struct TestStruct {
    map: MockMap,
}

impl TestStruct {
    fn unit_variant(mut self) -> Result<(), &'static str> {
        self.map.next_value()
    }
}

#[test]
fn test_unit_variant_success() {
    let test_struct = TestStruct {
        map: MockMap::new(false),
    };
    assert_eq!(test_struct.unit_variant(), Ok(()));
}

#[test]
fn test_unit_variant_error() {
    let test_struct = TestStruct {
        map: MockMap::new(true),
    };
    assert_eq!(test_struct.unit_variant(), Err("error"));
}

