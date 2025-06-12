// Answer 0

#[derive(Default)]
struct MapImpl {
    data: std::collections::HashMap<String, String>,
}

struct MyContainer {
    map: MapImpl,
}

impl MyContainer {
    pub fn new() -> Self {
        MyContainer {
            map: MapImpl::default(),
        }
    }
    
    pub fn append(&mut self, other: &mut Self) {
        #[cfg(feature = "preserve_order")]
        self.map
            .extend(std::mem::replace(&mut other.map, MapImpl::default()).data);
        #[cfg(not(feature = "preserve_order"))]
        self.map.data.extend(other.map.data.drain());
    }
}

#[test]
fn test_append_with_non_empty_other() {
    let mut container1 = MyContainer::new();
    container1.map.data.insert("key1".to_string(), "value1".to_string());

    let mut container2 = MyContainer::new();
    container2.map.data.insert("key2".to_string(), "value2".to_string());

    container1.append(&mut container2);

    assert_eq!(container1.map.data.len(), 2);
    assert_eq!(container1.map.data.get("key1").unwrap(), "value1");
    assert_eq!(container1.map.data.get("key2").unwrap(), "value2");
    assert!(container2.map.data.is_empty());
}

#[test]
fn test_append_with_empty_other() {
    let mut container1 = MyContainer::new();
    container1.map.data.insert("key1".to_string(), "value1".to_string());

    let mut container2 = MyContainer::new(); // empty

    container1.append(&mut container2);

    assert_eq!(container1.map.data.len(), 1);
    assert_eq!(container1.map.data.get("key1").unwrap(), "value1");
    assert!(container2.map.data.is_empty());
}

#[test]
fn test_append_preserve_order_feature() {
    #[cfg(feature = "preserve_order")]
    {
        let mut container1 = MyContainer::new();
        container1.map.data.insert("key1".to_string(), "value1".to_string());

        let mut container2 = MyContainer::new();
        container2.map.data.insert("key2".to_string(), "value2".to_string());

        container1.append(&mut container2);

        assert_eq!(container1.map.data.len(), 2);
        assert!(container2.map.data.is_empty());
    }
}

#[test]
fn test_append_identical_keys() {
    let mut container1 = MyContainer::new();
    container1.map.data.insert("key1".to_string(), "value1".to_string());

    let mut container2 = MyContainer::new();
    container2.map.data.insert("key1".to_string(), "value2".to_string());

    container1.append(&mut container2);

    assert_eq!(container1.map.data.len(), 1);
    assert_eq!(container1.map.data.get("key1").unwrap(), "value2");
    assert!(container2.map.data.is_empty());
}

#[should_panic]
#[test]
fn test_append_panic_on_empty_self() {
    let mut container1 = MyContainer::new(); // empty
    let mut container2 = MyContainer::new();
    container2.map.data.insert("key1".to_string(), "value1".to_string());

    container1.append(&mut container2); // This shouldn't panic in a normal context but is included to exemplify the panic handling
}

