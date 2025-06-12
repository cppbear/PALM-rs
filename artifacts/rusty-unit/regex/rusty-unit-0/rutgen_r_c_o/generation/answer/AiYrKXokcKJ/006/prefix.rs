// Answer 0

#[test]
fn test_add_byte_class_valid() {
    let mut literals = Literals::empty();
    literals.set_limit_size(100);
    literals.set_limit_class(50);
    
    let mut lit1 = Literal::empty();
    lit1.push(97); // 'a'
    literals.add(lit1);
    
    let mut lit2 = Literal::empty();
    lit2.push(98); // 'b'
    literals.add(lit2);
    
    let range = ClassBytesRange { start: 200, end: 210 }; // Valid range
    let class_bytes = ClassBytes::new(vec![range]);
    
    let result = literals.add_byte_class(&class_bytes);
}

#[test]
fn test_add_byte_class_base_not_empty() {
    let mut literals = Literals::empty();
    literals.set_limit_size(100);
    literals.set_limit_class(50);
    
    let mut lit1 = Literal::empty();
    lit1.push(100); // 'd'
    literals.add(lit1);
    
    let range = ClassBytesRange { start: 100, end: 110 }; // Valid range
    let class_bytes = ClassBytes::new(vec![range]);
    
    let result = literals.add_byte_class(&class_bytes);
}

#[test]
fn test_add_byte_class_large_range() {
    let mut literals = Literals::empty();
    literals.set_limit_size(100);
    literals.set_limit_class(50);
    
    let mut lit1 = Literal::empty();
    lit1.push(120); // 'x'
    literals.add(lit1);
    
    let range = ClassBytesRange { start: 1, end: 2 }; // Small range
    let class_bytes = ClassBytes::new(vec![range]);
    
    let result = literals.add_byte_class(&class_bytes);
}

#[test]
fn test_add_byte_class_exceeds_limit_class() {
    let mut literals = Literals::empty();
    literals.set_limit_size(10);
    literals.set_limit_class(5);
    
    let mut lit1 = Literal::empty();
    lit1.push(65); // 'A'
    literals.add(lit1);
    
    let range1 = ClassBytesRange { start: 0, end: 255 }; // Too large
    let class_bytes = ClassBytes::new(vec![range1]);
    
    let result = literals.add_byte_class(&class_bytes); // Should return false
}

#[test]
fn test_add_byte_class_empty_class() {
    let mut literals = Literals::empty();
    literals.set_limit_size(100);
    literals.set_limit_class(10);
    
    let mut lit1 = Literal::empty();
    lit1.push(99); // 'c'
    literals.add(lit1);
    
    let class_bytes = ClassBytes::empty(); // No ranges
    let result = literals.add_byte_class(&class_bytes); // Should return true
}

