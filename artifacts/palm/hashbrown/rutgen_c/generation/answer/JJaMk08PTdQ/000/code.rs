// Answer 0

#[test]
fn test_set_covariance() {
    use crate::hashbrown::HashSet;

    let set: HashSet<&'static str> = HashSet::new();
    let new_set: HashSet<&'new str> = set;
    assert!(new_set.is_empty());
}

#[test]
fn test_iter_covariance() {
    use crate::hashbrown::{HashSet, Iter};

    let set: HashSet<&'static str> = HashSet::new();
    let iter: Iter<'_, &'static str> = set.iter();
    let new_iter: Iter<'_, &'new str> = iter;
    assert!(new_iter.size_hint().0 == 0);
}

#[test]
fn test_into_iter_covariance() {
    use crate::hashbrown::{HashSet, IntoIter};

    let set: HashSet<&'static str> = HashSet::new();
    let into_iter: IntoIter<&'static str> = set.into_iter();
    let new_into_iter: IntoIter<&'new str> = into_iter;
    assert!(new_into_iter.len() == 0);
}

#[test]
fn test_difference_covariance() {
    use crate::hashbrown::{HashSet, Difference, DefaultHashBuilder};

    let set1: HashSet<&'static str> = HashSet::new();
    let set2: HashSet<&'static str> = HashSet::new();
    let difference: Difference<'_, &'static str, DefaultHashBuilder, _> = set1.difference(&set2);
    let new_difference: Difference<'_, &'new str, DefaultHashBuilder, _> = difference;
    assert!(new_difference.count() == 0);
}

#[test]
fn test_symmetric_difference_covariance() {
    use crate::hashbrown::{HashSet, SymmetricDifference, DefaultHashBuilder};

    let set1: HashSet<&'static str> = HashSet::new();
    let set2: HashSet<&'static str> = HashSet::new();
    let symmetric_difference: SymmetricDifference<'_, &'static str, DefaultHashBuilder, _> = set1.symmetric_difference(&set2);
    let new_symmetric_difference: SymmetricDifference<'_, &'new str, DefaultHashBuilder, _> = symmetric_difference;
    assert!(new_symmetric_difference.count() == 0);
}

#[test]
fn test_intersection_covariance() {
    use crate::hashbrown::{HashSet, Intersection, DefaultHashBuilder};

    let set1: HashSet<&'static str> = HashSet::new();
    let set2: HashSet<&'static str> = HashSet::new();
    let intersection: Intersection<'_, &'static str, DefaultHashBuilder, _> = set1.intersection(&set2);
    let new_intersection: Intersection<'_, &'new str, DefaultHashBuilder, _> = intersection;
    assert!(new_intersection.count() == 0);
}

#[test]
fn test_union_covariance() {
    use crate::hashbrown::{HashSet, Union, DefaultHashBuilder};

    let set1: HashSet<&'static str> = HashSet::new();
    let set2: HashSet<&'static str> = HashSet::new();
    let union: Union<'_, &'static str, DefaultHashBuilder, _> = set1.union(&set2);
    let new_union: Union<'_, &'new str, DefaultHashBuilder, _> = union;
    assert!(new_union.count() == 0);
}

#[test]
fn test_drain_covariance() {
    use crate::hashbrown::{HashSet, Drain};

    let set: HashSet<&'static str> = HashSet::new();
    let drain: Drain<'static, &'static str, _> = set.drain();
    let new_drain: Drain<'new, &'new str, _> = drain;
    assert!(new_drain.count() == 0);
}

