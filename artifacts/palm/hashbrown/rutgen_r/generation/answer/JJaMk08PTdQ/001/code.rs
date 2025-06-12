// Answer 0

#[test]
fn test_set() {
    let input: HashSet<&'static str> = HashSet::from(["apple", "banana", "cherry"].map(|s| s));
    let output: HashSet<&str> = set(input);
    assert_eq!(output.len(), 3);
}

#[test]
fn test_iter() {
    let input: HashSet<&'static str> = HashSet::from(["apple", "banana", "cherry"].map(|s| s));
    let iter_input = input.iter();
    let iter_output: Iter<&str> = iter(iter_input);
    assert_eq!(iter_output.len(), 3);
}

#[test]
fn test_into_iter() {
    let input: HashSet<&'static str> = HashSet::from(["apple", "banana", "cherry"].map(|s| s));
    let into_iter_input = input.into_iter();
    let into_iter_output: IntoIter<&str> = into_iter(into_iter_input);
    assert_eq!(into_iter_output.len(), 3);
}

#[test]
fn test_difference() {
    let set_a: HashSet<&'static str> = HashSet::from(["apple", "banana"]);
    let set_b: HashSet<&'static str> = HashSet::from(["banana", "cherry"]);
    let difference_input = set_a.difference(&set_b);
    let difference_output: Difference<&str, DefaultHashBuilder, _> = difference(difference_input);
    assert_eq!(difference_output.len(), 1);
}

#[test]
fn test_symmetric_difference() {
    let set_a: HashSet<&'static str> = HashSet::from(["apple", "banana"]);
    let set_b: HashSet<&'static str> = HashSet::from(["banana", "cherry"]);
    let symmetric_difference_input = set_a.symmetric_difference(&set_b);
    let symmetric_difference_output: SymmetricDifference<&str, DefaultHashBuilder, _> = symmetric_difference(symmetric_difference_input);
    assert_eq!(symmetric_difference_output.len(), 2);
}

#[test]
fn test_intersection() {
    let set_a: HashSet<&'static str> = HashSet::from(["apple", "banana"]);
    let set_b: HashSet<&'static str> = HashSet::from(["banana", "cherry"]);
    let intersection_input = set_a.intersection(&set_b);
    let intersection_output: Intersection<&str, DefaultHashBuilder, _> = intersection(intersection_input);
    assert_eq!(intersection_output.len(), 1);
}

#[test]
fn test_union() {
    let set_a: HashSet<&'static str> = HashSet::from(["apple", "banana"]);
    let set_b: HashSet<&'static str> = HashSet::from(["banana", "cherry"]);
    let union_input = set_a.union(&set_b);
    let union_output: Union<&str, DefaultHashBuilder, _> = union(union_input);
    assert_eq!(union_output.len(), 3);
}

#[test]
fn test_drain() {
    let mut input: HashSet<&'static str> = HashSet::from(["apple", "banana", "cherry"]);
    let drain_input = input.drain();
    let drain_output: Drain<&'static str, _> = drain(drain_input);
    assert_eq!(drain_output.len(), 3);
}

