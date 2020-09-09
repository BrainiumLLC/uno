use uno::*;

#[test]
fn zero_and_one() {
    assert_eq!(Unorm8::from_float(0.0), Unorm8::zero());
    assert_eq!(Unorm8::from_float(1.0), Unorm8::one());
}

#[test]
fn there_and_back_again() {
    const STEPS: u8 = Unorm8::max_value().to_inner();
    for n in 0..=STEPS {
        let i = n as f32 / STEPS as f32;
        let o: f32 = Unorm8::from_float(i).to_float();
        if n == 1 {
            assert_eq!(o, Unorm8::epsilon().to_float());
        }
        assert_eq!(i, o, "expected {:?}, got {:?}", i, o);
    }
}
