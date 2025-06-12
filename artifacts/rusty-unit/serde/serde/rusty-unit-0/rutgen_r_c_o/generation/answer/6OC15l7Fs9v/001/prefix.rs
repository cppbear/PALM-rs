// Answer 0

#[test]
fn test_serialize_seq_none() {
    let mut map = MyMap::new();
    let serializer = FlatMapSerializer(&mut map);
    serializer.serialize_seq(None);
}

#[test]
fn test_serialize_seq_zero() {
    let mut map = MyMap::new();
    let serializer = FlatMapSerializer(&mut map);
    serializer.serialize_seq(Some(0));
}

#[test]
fn test_serialize_seq_one() {
    let mut map = MyMap::new();
    let serializer = FlatMapSerializer(&mut map);
    serializer.serialize_seq(Some(1));
}

#[test]
fn test_serialize_seq_two() {
    let mut map = MyMap::new();
    let serializer = FlatMapSerializer(&mut map);
    serializer.serialize_seq(Some(2));
}

#[test]
fn test_serialize_seq_three() {
    let mut map = MyMap::new();
    let serializer = FlatMapSerializer(&mut map);
    serializer.serialize_seq(Some(3));
}

#[test]
fn test_serialize_seq_four() {
    let mut map = MyMap::new();
    let serializer = FlatMapSerializer(&mut map);
    serializer.serialize_seq(Some(4));
}

#[test]
fn test_serialize_seq_five() {
    let mut map = MyMap::new();
    let serializer = FlatMapSerializer(&mut map);
    serializer.serialize_seq(Some(5));
}

#[test]
fn test_serialize_seq_ten() {
    let mut map = MyMap::new();
    let serializer = FlatMapSerializer(&mut map);
    serializer.serialize_seq(Some(10));
}

#[test]
fn test_serialize_seq_hundred() {
    let mut map = MyMap::new();
    let serializer = FlatMapSerializer(&mut map);
    serializer.serialize_seq(Some(100));
}

#[test]
fn test_serialize_seq_thousand() {
    let mut map = MyMap::new();
    let serializer = FlatMapSerializer(&mut map);
    serializer.serialize_seq(Some(1_000));
}

#[test]
fn test_serialize_seq_ten_thousand() {
    let mut map = MyMap::new();
    let serializer = FlatMapSerializer(&mut map);
    serializer.serialize_seq(Some(10_000));
} 

#[test]
fn test_serialize_seq_max_usize() {
    let mut map = MyMap::new();
    let serializer = FlatMapSerializer(&mut map);
    serializer.serialize_seq(Some(usize::MAX));
}

