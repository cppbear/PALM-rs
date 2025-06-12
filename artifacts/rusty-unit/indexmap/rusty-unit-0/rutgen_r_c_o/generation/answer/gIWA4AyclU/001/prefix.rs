// Answer 0

#[test]
fn test_append_with_non_empty_a_and_non_empty_b() {
    let mut a = IndexSet::<i32, RandomState>::from([3, 2, 1]);
    let mut b = IndexSet::<i32, RandomState>::from([3, 4, 5]);
    
    a.append(&mut b);
    
    let _ = a.len();
    let _ = b.len();
    let _ = b.capacity();
    let _ = a.iter().collect::<Vec<_>>();
}

#[test]
fn test_append_with_empty_a_and_non_empty_b() {
    let mut a = IndexSet::<i32, RandomState>::new();
    let mut b = IndexSet::<i32, RandomState>::from([3, 4, 5]);
    
    a.append(&mut b);
    
    let _ = a.len();
    let _ = b.len();
    let _ = b.capacity();
    let _ = a.iter().collect::<Vec<_>>();
}

#[test]
fn test_append_with_non_empty_a_and_empty_b() {
    let mut a = IndexSet::<i32, RandomState>::from([3, 2, 1]);
    let mut b = IndexSet::<i32, RandomState>::new();
    
    a.append(&mut b);
    
    let _ = a.len();
    let _ = b.len();
    let _ = b.capacity();
    let _ = a.iter().collect::<Vec<_>>();
}

#[test]
fn test_append_with_empty_a_and_empty_b() {
    let mut a = IndexSet::<i32, RandomState>::new();
    let mut b = IndexSet::<i32, RandomState>::new();
    
    a.append(&mut b);
    
    let _ = a.len();
    let _ = b.len();
    let _ = b.capacity();
    let _ = a.iter().collect::<Vec<_>>();
}

#[test]
fn test_append_with_large_inputs() {
    let mut a = IndexSet::<i32, RandomState>::from_iter(0..500);
    let mut b = IndexSet::<i32, RandomState>::from_iter(500..1000);
    
    a.append(&mut b);
    
    let _ = a.len();
    let _ = b.len();
    let _ = b.capacity();
    let _ = a.iter().collect::<Vec<_>>();
}

