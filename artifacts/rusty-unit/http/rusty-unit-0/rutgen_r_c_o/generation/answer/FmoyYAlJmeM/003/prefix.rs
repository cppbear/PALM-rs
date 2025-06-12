// Answer 0

#[test]
fn test_extend_with_non_empty_maps() {
    let mut ext_a = Extensions::new();
    ext_a.insert(1u8);
    ext_a.insert(2u16);

    let mut ext_b = Extensions::new();
    ext_b.insert(3u8);
    ext_b.insert("test");

    ext_a.extend(ext_b);
}

#[test]
fn test_extend_with_overlapping_types() {
    let mut ext_a = Extensions::new();
    ext_a.insert(5u8);
    ext_a.insert(10u16);
    
    let mut ext_b = Extensions::new();
    ext_b.insert(6u8);
    ext_b.insert(20u16);
    ext_b.insert("world");

    ext_a.extend(ext_b);
}

#[test]
fn test_extend_with_empty_self() {
    let mut ext_a = Extensions::new();

    let mut ext_b = Extensions::new();
    ext_b.insert(8u8);
    ext_b.insert(16u16);

    ext_a.extend(ext_b);
}

#[test]
fn test_extend_with_empty_other() {
    let mut ext_a = Extensions::new();
    ext_a.insert(4u8);
    ext_a.insert("hello");

    let ext_b = Extensions::new();

    ext_a.extend(ext_b);
}

#[test]
fn test_extend_with_multiple_types() {
    let mut ext_a = Extensions::new();
    ext_a.insert(1u8);
    ext_a.insert(42u64);
    
    let mut ext_b = Extensions::new();
    ext_b.insert(100u16);
    ext_b.insert("example");
    ext_b.insert(3.14f64);
    
    ext_a.extend(ext_b);
}

#[test]
fn test_extend_with_max_size() {
    let mut ext_a = Extensions::new();
    for i in 0..10 {
        ext_a.insert(i as u8);
    }

    let mut ext_b = Extensions::new();
    for j in 10..20 {
        ext_b.insert(j as u8);
    }

    ext_a.extend(ext_b);
}

