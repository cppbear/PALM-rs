// Answer 0

#[derive(Debug)]
struct MyMap {
    key: String,
}

trait KeyMut {
    type Key;
    fn key_mut(&mut self) -> &mut Self::Key;
}

impl KeyMut for MyMap {
    type Key = String;
    
    fn key_mut(&mut self) -> &mut Self::Key {
        &mut self.key
    }
}

#[test]
fn test_key_mut() {
    let mut my_map = MyMap {
        key: String::from("initial"),
    };

    {
        let key_ref = my_map.key_mut();
        *key_ref = String::from("modified");
    }

    assert_eq!(my_map.key, "modified");
}

#[test]
fn test_key_mut_empty() {
    let mut my_map = MyMap {
        key: String::new(),
    };

    {
        let key_ref = my_map.key_mut();
        *key_ref = String::from("new_key");
    }

    assert_eq!(my_map.key, "new_key");
}

