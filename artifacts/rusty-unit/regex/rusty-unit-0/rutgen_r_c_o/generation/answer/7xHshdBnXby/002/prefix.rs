// Answer 0

#[test]
fn test_literal_byte_0x80() {
    let lit = Literal::Byte(0x80);
    let result = Hir::literal(lit);
}

#[test]
fn test_literal_byte_0xFF() {
    let lit = Literal::Byte(0xFF);
    let result = Hir::literal(lit);
}

#[test]
fn test_literal_unicode_character_a() {
    let lit = Literal::Unicode('a');
    let result = Hir::literal(lit);
}

#[test]
fn test_literal_unicode_character_z() {
    let lit = Literal::Unicode('z');
    let result = Hir::literal(lit);
}

#[test]
fn test_literal_unicode_character_unicode() {
    let lit = Literal::Unicode('ö');
    let result = Hir::literal(lit);
}

