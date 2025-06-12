// Answer 0

#[derive(Debug)]
struct MockAstGroup {
    kind: ast::GroupKind,
}

#[derive(Debug)]
struct MockHir;

impl MockHir {
    pub fn group(group: hir::Group) -> Hir {
        Hir {
            group,
        }
    }
}

#[derive(Debug)]
struct Hir {
    group: hir::Group,
}

#[test]
fn test_hir_group_capture_index() {
    // Arrange
    let group = MockAstGroup {
        kind: ast::GroupKind::CaptureIndex(0),
    };
    let expr = MockHir {};

    // Act
    let result = hir_group(&group, expr);

    // Assert
    assert_eq!(result.group.kind, hir::GroupKind::CaptureIndex(0));
}

#[test]
fn test_hir_group_capture_name() {
    // Arrange
    let capname = ast::CaptureName {
        name: "group_name".to_string(),
        index: 1,
    };
    let group = MockAstGroup {
        kind: ast::GroupKind::CaptureName(capname),
    };
    let expr = MockHir {};

    // Act
    let result = hir_group(&group, expr);

    // Assert
    match result.group.kind {
        hir::GroupKind::CaptureName { name, index } => {
            assert_eq!(name, "group_name");
            assert_eq!(index, 1);
        },
        _ => panic!("Expected CaptureName variant."),
    }
}

#[test]
fn test_hir_group_non_capturing() {
    // Arrange
    let group = MockAstGroup {
        kind: ast::GroupKind::NonCapturing(()),
    };
    let expr = MockHir {};

    // Act
    let result = hir_group(&group, expr);

    // Assert
    assert_eq!(result.group.kind, hir::GroupKind::NonCapturing);
}

