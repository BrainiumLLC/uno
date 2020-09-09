use en::num_traits::{bounds::Bounded, sign::Unsigned};
use std::fmt::Debug;
use thiserror::Error;

pub fn extend_unorm<A, B>(n: A) -> B
where
    A: en::Num + Bounded + Unsigned,
    B: From<A> + en::Num + Bounded + Unsigned,
{
    let steps_a: B = A::max_value().into();
    let steps_b = B::max_value();
    let factor: B = en::cast(steps_b / steps_a);
    <B as From<A>>::from(n) * factor
}

#[derive(Debug, Error)]
pub enum FloatToUnormErrorSource {
    #[error("float is less than zero")]
    LtZero,
    #[error("float is greater than one")]
    GtOne,
    #[error("float is not a number")]
    NaN,
}

#[derive(Debug, Error)]
#[error("couldn't convert float {value:?} to unorm: {source}")]
pub struct FloatToUnormError<F: en::Float> {
    pub value: F,
    pub source: FloatToUnormErrorSource,
}

impl<F: en::Float> FloatToUnormError<F> {
    fn new(value: F, source: FloatToUnormErrorSource) -> Self {
        Self { value, source }
    }

    fn lt_zero(value: F) -> Self {
        Self::new(value, FloatToUnormErrorSource::LtZero)
    }

    fn gt_one(value: F) -> Self {
        Self::new(value, FloatToUnormErrorSource::GtOne)
    }

    fn nan(value: F) -> Self {
        Self::new(value, FloatToUnormErrorSource::NaN)
    }
}

// Referenced from https://docs.microsoft.com/en-us/windows/win32/direct3d10/d3d10-graphics-programming-guide-resources-data-conversion#integer-conversion

pub fn float_to_unorm<F: en::Float, N: en::Num + Bounded + Unsigned>(
    f: F,
) -> Result<N, FloatToUnormError<F>> {
    if f.is_nan() {
        Err(FloatToUnormError::nan(f))
    } else if f < F::zero() {
        Err(FloatToUnormError::lt_zero(f))
    } else if f > F::one() {
        Err(FloatToUnormError::gt_one(f))
    } else {
        let scale: F = en::cast(N::max_value());
        Ok(en::cast((f * scale + F::one().halved()).trunc()))
    }
}

pub fn float_to_unorm_clamped<F: en::Float, N: en::Num + Bounded + Unsigned>(f: F) -> N {
    float_to_unorm(if f.is_nan() {
        F::zero()
    } else {
        en::Max::max(en::Min::min(f, F::one()), F::zero())
    })
    .expect("`float_to_unorm` call in `float_to_unorm_clamped` failed, which should never happen")
}

pub fn denominator_to_unorm<N: en::Num + Unsigned>(denominator: N) -> N {
    N::one() / denominator
}

pub fn unorm_to_float<N: en::Num + Bounded + Unsigned, F: en::Float>(n: N) -> F {
    en::cast::<F, _>(n) / en::cast::<F, _>(N::max_value())
}
