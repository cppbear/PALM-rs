// Answer 0

#[derive(Debug)]
struct ExampleKey;

struct ExampleMap {
    key: ExampleKey,
}

impl ExampleMap {
    fn key_mut(&mut self) -> &mut ExampleKey {
        &mut self.key
    }
}

#[test]
fn test_key_mut() {
    let mut map = ExampleMap { key: ExampleKey };

    let key_ref = map.key_mut();
    *key_ref = ExampleKey;

    assert!(std::mem::size_of_val(key_ref) == std::mem::size_of::<ExampleKey>());
}

