// Answer 0

#[test]
#[should_panic]
fn test_advance_mut_panic_over_advance() {
    // Arrange
    let mut bytes_mut = BytesMut::with_capacity(5);
    unsafe { bytes_mut.set_len(2); } // Set len to 2
    // remaining = cap (5) - len (2) = 3

    // Act
    unsafe { bytes_mut.advance_mut(4); } // cnt (4) > remaining (3)
}

#[test]
#[should_panic]
fn test_advance_mut_panic_exact_capacity() {
    // Arrange
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe { bytes_mut.set_len(7); } // Set len to 7
    // remaining = cap (10) - len (7) = 3

    // Act
    unsafe { bytes_mut.advance_mut(4); } // cnt (4) > remaining (3)
}

#[test]
#[should_panic]
fn test_advance_mut_panic_larger_capacity() {
    // Arrange
    let mut bytes_mut = BytesMut::with_capacity(8);
    unsafe { bytes_mut.set_len(3); } // Set len to 3
    // remaining = cap (8) - len (3) = 5

    // Act
    unsafe { bytes_mut.advance_mut(6); } // cnt (6) > remaining (5)
}

#[test]
fn test_advance_mut_no_panic() {
    // Arrange
    let mut bytes_mut = BytesMut::with_capacity(12);
    unsafe { bytes_mut.set_len(5); } // Set len to 5
    // remaining = cap (12) - len (5) = 7

    // Act
    unsafe { bytes_mut.advance_mut(3); } // cnt (3) <= remaining (7)

    // Assert
    assert_eq!(bytes_mut.len(), 8); // New len should be 5 + 3
}

