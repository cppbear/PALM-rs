// Answer 0

#[test]
fn test_extend_with_non_empty_extensions() {
    use std::any::{Any, TypeId};
    use std::collections::HashMap;

    #[derive(Clone)]
    struct AnyClone;

    impl AnyClone {
        fn new<T: Clone + Send + Sync + 'static>(val: T) -> Box<dyn AnyClone + Send + Sync> {
            Box::new(val) as Box<dyn AnyClone + Send + Sync>
        }
    }

    // Define the helper structure for our tests
    let mut ext_a = Extensions::new();
    ext_a.insert(8u8);
    ext_a.insert(16u16);

    let mut ext_b = Extensions::new();
    ext_b.insert(4u8);
    ext_b.insert("hello");

    ext_a.extend(ext_b);
    
    assert_eq!(ext_a.len(), 3);
    assert_eq!(ext_a.get::<u8>(), Some(&4u8));
    assert_eq!(ext_a.get::<u16>(), Some(&16u16));
    assert_eq!(ext_a.get::<&'static str>().copied(), Some("hello"));
}

#[test]
fn test_extend_with_overwriting_existing_type() {
    use std::any::{Any, TypeId};
    use std::collections::HashMap;

    #[derive(Clone)]
    struct AnyClone;

    impl AnyClone {
        fn new<T: Clone + Send + Sync + 'static>(val: T) -> Box<dyn AnyClone + Send + Sync> {
            Box::new(val) as Box<dyn AnyClone + Send + Sync>
        }
    }

    let mut ext_a = Extensions::new();
    ext_a.insert(1u8);
    
    let mut ext_b = Extensions::new();
    ext_b.insert(2u8);
    
    ext_a.extend(ext_b);

    assert_eq!(ext_a.len(), 1);
    assert_eq!(ext_a.get::<u8>(), Some(&2u8));
}

#[test]
fn test_extend_with_no_common_types() {
    use std::any::{Any, TypeId};
    use std::collections::HashMap;

    #[derive(Clone)]
    struct AnyClone;

    impl AnyClone {
        fn new<T: Clone + Send + Sync + 'static>(val: T) -> Box<dyn AnyClone + Send + Sync> {
            Box::new(val) as Box<dyn AnyClone + Send + Sync>
        }
    }

    let mut ext_a = Extensions::new();
    ext_a.insert(10u16);
    
    let mut ext_b = Extensions::new();
    ext_b.insert("world");

    ext_a.extend(ext_b);
    
    assert_eq!(ext_a.len(), 2);
    assert_eq!(ext_a.get::<u16>(), Some(&10u16));
    assert_eq!(ext_a.get::<&str>().copied(), Some("world"));
}

#[test]
fn test_extend_empty_with_non_empty_extensions() {
    let mut ext_a = Extensions::new();
    let mut ext_b = Extensions::new();
    ext_b.insert(1u8);
    
    ext_a.extend(ext_b);

    assert_eq!(ext_a.len(), 1);
    assert_eq!(ext_a.get::<u8>(), Some(&1u8));
}

#[test]
fn test_extend_with_empty_other() {
    let mut ext_a = Extensions::new();
    ext_a.insert(5u32);
    
    let ext_b = Extensions::new();
    ext_a.extend(ext_b);

    assert_eq!(ext_a.len(), 1);
    assert_eq!(ext_a.get::<u32>(), Some(&5u32));
}

