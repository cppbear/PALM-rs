// Answer 0

#[test]
fn test_longest_common_prefix_case_1() {
    let mut literals = Literals::empty();
    literals.add(Literal::Byte(97)); // 'a'
    literals.add(Literal::Byte(97)); // 'a'
    literals.add(Literal::Byte(97)); // 'a'
    literals.add(Literal::Byte(98)); // 'b'
    literals.add(Literal::Byte(99)); // 'c'
    let _ = literals.longest_common_prefix();
}

#[test]
fn test_longest_common_prefix_case_2() {
    let mut literals = Literals::empty();
    literals.add(Literal::Byte(100)); // 'd'
    literals.add(Literal::Byte(100)); // 'd'
    literals.add(Literal::Byte(100)); // 'd'
    literals.add(Literal::Byte(101)); // 'e'
    literals.add(Literal::Byte(102)); // 'f'
    let _ = literals.longest_common_prefix();
}

#[test]
fn test_longest_common_prefix_case_3() {
    let mut literals = Literals::empty();
    literals.add(Literal::Byte(103)); // 'g'
    literals.add(Literal::Byte(103)); // 'g'
    literals.add(Literal::Byte(103)); // 'g'
    literals.add(Literal::Byte(104)); // 'h'
    literals.add(Literal::Byte(105)); // 'i'
    let _ = literals.longest_common_prefix();
}

#[test]
fn test_longest_common_prefix_case_4() {
    let mut literals = Literals::empty();
    literals.add(Literal::Byte(100)); // 'd'
    literals.add(Literal::Byte(100)); // 'd'
    literals.add(Literal::Byte(101)); // 'e'
    literals.add(Literal::Byte(103)); // 'g'
    literals.add(Literal::Byte(104)); // 'h'
    let _ = literals.longest_common_prefix();
}

#[test]
fn test_longest_common_prefix_edge_case() {
    let mut literals = Literals::empty();
    literals.add(Literal::Byte(1)); // byte default
    literals.add(Literal::Byte(1)); // byte default
    literals.add(Literal::Byte(1)); // byte default
    literals.add(Literal::Byte(2)); // distinct byte
    literals.add(Literal::Byte(3)); // distinct byte
    let _ = literals.longest_common_prefix();
}

