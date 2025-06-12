// Answer 0

#[test]
fn test_hashmap_debug_fmt_empty() {
    use crate::{HashMap, DefaultHashBuilder, Global};

    let map: HashMap<i32, &str, DefaultHashBuilder, Global> = HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder::new(), Global);
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", map);
    assert_eq!(output.trim(), "{}");
}

#[test]
fn test_hashmap_debug_fmt_non_empty() {
    use crate::{HashMap, DefaultHashBuilder, Global};

    let mut map: HashMap<i32, &str, DefaultHashBuilder, Global> = HashMap::with_capacity_and_hasher_in(2, DefaultHashBuilder::new(), Global);
    map.insert(1, "one");
    map.insert(2, "two");

    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", map);
    assert!(output.contains("1"));
    assert!(output.contains("two"));
}

#[test]
fn test_hashmap_debug_fmt_boundary_conditions() {
    use crate::{HashMap, DefaultHashBuilder, Global};

    let mut map: HashMap<i32, &str, DefaultHashBuilder, Global> = HashMap::with_capacity_and_hasher_in(1, DefaultHashBuilder::new(), Global);
    map.insert(0, "zero");

    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", map);
    assert!(output.contains("zero"));

    map.clear();
    let _ = write!(&mut output, "{:?}", map);
    assert_eq!(output.trim(), "{}");
}

