// Answer 0

#[test]
fn test_set_covariance() {
    use std::collections::HashSet;

    let set: HashSet<&str> = {
        let mut temp_set = HashSet::new();
        temp_set.insert("foo");
        temp_set.insert("bar");
        temp_set
    };

    let new_set: HashSet<&str> = set(set);
    assert!(new_set.contains("foo"));
    assert!(new_set.contains("bar"));
}

#[test]
fn test_iter_covariance() {
    use std::collections::HashSet;
    use std::iter::FromIterator;

    let set: HashSet<&str> = HashSet::from_iter(vec!["foo", "bar"]);
    let mut iter = iter(set.iter());

    assert_eq!(iter.next(), Some(&"foo"));
    assert_eq!(iter.next(), Some(&"bar"));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_into_iter_covariance() {
    use std::collections::HashSet;

    let set: HashSet<&str> = {
        let mut temp_set = HashSet::new();
        temp_set.insert("foo");
        temp_set.insert("bar");
        temp_set
    };

    let mut into_iter = into_iter(set.into_iter());

    assert_eq!(into_iter.next(), Some("foo"));
    assert_eq!(into_iter.next(), Some("bar"));
    assert_eq!(into_iter.next(), None);
}

#[test]
fn test_difference_covariance() {
    use std::collections::HashSet;

    let set_a: HashSet<&str> = {
        let mut temp_set = HashSet::new();
        temp_set.insert("foo");
        temp_set.insert("bar");
        temp_set
    };
    
    let set_b: HashSet<&str> = {
        let mut temp_set = HashSet::new();
        temp_set.insert("bar");
        temp_set
    };

    let difference = difference(set_a.difference(&set_b));
    
    let expected_diff: HashSet<&str> = {
        let mut temp_set = HashSet::new();
        temp_set.insert("foo");
        temp_set
    };
    
    assert_eq!(difference.collect::<HashSet<_>>(), expected_diff);
}

#[test]
fn test_symmetric_difference_covariance() {
    use std::collections::HashSet;

    let set_a: HashSet<&str> = {
        let mut temp_set = HashSet::new();
        temp_set.insert("foo");
        temp_set.insert("bar");
        temp_set
    };
    
    let set_b: HashSet<&str> = {
        let mut temp_set = HashSet::new();
        temp_set.insert("bar");
        temp_set.insert("baz");
        temp_set
    };

    let symmetric_diff = symmetric_difference(set_a.symmetric_difference(&set_b));

    let expected_diff: HashSet<&str> = {
        let mut temp_set = HashSet::new();
        temp_set.insert("foo");
        temp_set.insert("baz");
        temp_set
    };
    
    assert_eq!(symmetric_diff.collect::<HashSet<_>>(), expected_diff);
}

#[test]
fn test_intersection_covariance() {
    use std::collections::HashSet;

    let set_a: HashSet<&str> = {
        let mut temp_set = HashSet::new();
        temp_set.insert("foo");
        temp_set.insert("bar");
        temp_set
    };
    
    let set_b: HashSet<&str> = {
        let mut temp_set = HashSet::new();
        temp_set.insert("bar");
        temp_set.insert("baz");
        temp_set
    };

    let intersection = intersection(set_a.intersection(&set_b));

    let expected_intersection: HashSet<&str> = {
        let mut temp_set = HashSet::new();
        temp_set.insert("bar");
        temp_set
    };
    
    assert_eq!(intersection.collect::<HashSet<_>>(), expected_intersection);
}

#[test]
fn test_union_covariance() {
    use std::collections::HashSet;

    let set_a: HashSet<&str> = {
        let mut temp_set = HashSet::new();
        temp_set.insert("foo");
        temp_set.insert("bar");
        temp_set
    };
    
    let set_b: HashSet<&str> = {
        let mut temp_set = HashSet::new();
        temp_set.insert("bar");
        temp_set.insert("baz");
        temp_set
    };

    let union = union(set_a.union(&set_b));

    let expected_union: HashSet<&str> = {
        let mut temp_set = HashSet::new();
        temp_set.insert("foo");
        temp_set.insert("bar");
        temp_set.insert("baz");
        temp_set
    };
    
    assert_eq!(union.collect::<HashSet<_>>(), expected_union);
} 

#[test]
fn test_drain_covariance() {
    use std::collections::HashSet;

    let mut set: HashSet<&str> = {
        let mut temp_set = HashSet::new();
        temp_set.insert("foo");
        temp_set.insert("bar");
        temp_set
    };

    let mut drain = drain(set.drain());

    let drained_elements: HashSet<&str> = drain.collect();

    assert!(drained_elements.contains("foo"));
    assert!(drained_elements.contains("bar"));
    assert!(set.is_empty());
}

