use uno::*;

fn gimme_both(n: Unorm8) -> (f32, f32) {
    let converted = Unorm64::from(n);
    (n.to_float(), converted.to_float())
}

#[test]
fn just_one() {
    let (unorm8, unorm64) = gimme_both(Unorm8::one());
    assert_eq!(unorm8, unorm64);
}

#[test]
fn digital_style() {
    let (unorm8, unorm64) = gimme_both(Unorm8::from_float(0.123456789));
    assert_eq!(unorm8, unorm64);
}
