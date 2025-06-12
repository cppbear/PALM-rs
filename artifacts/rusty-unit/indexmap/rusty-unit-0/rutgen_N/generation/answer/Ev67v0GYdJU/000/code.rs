// Answer 0

#[derive(Debug)]
struct Entry<V> {
    value: V,
}

impl<V> Entry<V> {
    pub fn get_mut(&mut self) -> &mut V {
        &mut self.value
    }

    pub fn insert(&mut self, value: V) -> V {
        std::mem::replace(self.get_mut(), value)
    }
}

#[test]
fn test_insert_replaces_value_and_returns_old_value() {
    let mut entry = Entry { value: 42 };

    let old_value = entry.insert(100);
    
    assert_eq!(old_value, 42);
    assert_eq!(entry.value, 100);
}

#[test]
fn test_insert_with_different_types() {
    struct MyStruct {
        data: i32,
    }

    let mut entry = Entry { value: MyStruct { data: 1 } };

    let old_value = entry.insert(MyStruct { data: 2 });

    assert_eq!(old_value.data, 1);
    assert_eq!(entry.value.data, 2);
}

#[test]
fn test_insert_same_value() {
    let mut entry = Entry { value: "original" };

    let old_value = entry.insert("original");
    
    assert_eq!(old_value, "original");
    assert_eq!(entry.value, "original");
}

#[test]
fn test_insert_empty_value() {
    let mut entry = Entry { value: String::from("initial") };

    let old_value = entry.insert(String::from(""));
    
    assert_eq!(old_value, "initial");
    assert_eq!(entry.value, "");
}

