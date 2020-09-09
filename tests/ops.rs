use uno::*;

#[test]
fn add() {
    assert_eq!(
        Unorm8::from_float(0.5),
        Unorm8::from_float(0.2) + Unorm8::from_float(0.3)
    );
}
