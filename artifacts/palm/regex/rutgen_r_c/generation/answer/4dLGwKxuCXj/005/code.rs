// Answer 0

#[test]
fn test_pos_valid_capture_group() {
    // Arrange
    let locations = Locations(vec![Some(0), Some(5), None, Some(10), Some(15)]);

    // Act
    let result = locations.pos(1); // Should return (5, 10)

    // Assert
    assert_eq!(result, Some((5, 10)));
}

#[test]
fn test_pos_invalid_capture_group_high_index() {
    // Arrange
    let locations = Locations(vec![Some(0), Some(5), None, Some(10), Some(15)]);

    // Act
    let result = locations.pos(3); // Index 3 has no valid end position

    // Assert
    assert_eq!(result, None);
}

#[test]
fn test_pos_invalid_capture_group_low_index() {
    // Arrange
    let locations = Locations(vec![Some(0), Some(5), None, Some(10), Some(15)]);

    // Act
    let result = locations.pos(0); // Should return (0, 5)

    // Assert
    assert_eq!(result, Some((0, 5)));
}

#[test]
fn test_pos_no_capture_group() {
    // Arrange
    let locations = Locations(vec![None, None, None, None]);

    // Act
    let result = locations.pos(0); // No valid positions

    // Assert
    assert_eq!(result, None);
}

#[test]
fn test_pos_empty_locations() {
    // Arrange
    let locations = Locations(vec![]);

    // Act
    let result = locations.pos(0); // No elements available

    // Assert
    assert_eq!(result, None);
}

