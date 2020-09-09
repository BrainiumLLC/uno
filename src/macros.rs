macro_rules! ops {
    ($struct:ident, $rhs:ty) => {
        impl std::ops::Add<$rhs> for $struct {
            type Output = Self;

            fn add(self, other: $rhs) -> Self {
                self.map(|inner| inner + other.to_inner())
            }
        }

        impl std::ops::AddAssign<$rhs> for $struct {
            fn add_assign(&mut self, other: $rhs) {
                *self = *self + other;
            }
        }

        impl std::ops::Sub<$rhs> for $struct {
            type Output = Self;

            fn sub(self, other: $rhs) -> Self {
                self.map(|inner| inner - other.to_inner())
            }
        }

        impl std::ops::SubAssign<$rhs> for $struct {
            fn sub_assign(&mut self, other: $rhs) {
                *self = *self - other;
            }
        }

        impl std::ops::Mul<$rhs> for $struct {
            type Output = Self;

            fn mul(self, other: $rhs) -> Self {
                self.map(|inner| inner * other.to_inner())
            }
        }

        impl std::ops::MulAssign<$rhs> for $struct {
            fn mul_assign(&mut self, other: $rhs) {
                *self = *self * other;
            }
        }

        impl std::ops::Div<$rhs> for $struct {
            type Output = Self;

            fn div(self, other: $rhs) -> Self {
                self.map(|inner| inner / other.to_inner())
            }
        }

        impl std::ops::DivAssign<$rhs> for $struct {
            fn div_assign(&mut self, other: $rhs) {
                *self = *self / other;
            }
        }

        impl std::ops::Rem<$rhs> for $struct {
            type Output = Self;

            fn rem(self, other: $rhs) -> Self {
                self.map(|inner| inner % other.to_inner())
            }
        }

        impl std::ops::RemAssign<$rhs> for $struct {
            fn rem_assign(&mut self, other: $rhs) {
                *self = *self % other;
            }
        }
    };
}

macro_rules! special_ops {
    ($inner:ty, $op:ident, Self) => {
        pub fn $op(self, other: Self) -> Self {
            self.map(|inner| <$inner>::$op(inner, other.to_inner()))
        }
    };

    ($inner:ty, $op:ident) => {
        pub fn $op(self) -> Self {
            self.map(|inner| <$inner>::$op(inner))
        }
    };

    ($inner:ty, $op:ident, $rhs:ty) => {
        pub fn $op(self, other: $rhs) -> Self {
            self.map(|inner| <$inner>::$op(inner, other))
        }
    };

    ($inner:ty) => {
        special_ops!($inner, saturating_add, Self);
        special_ops!($inner, saturating_mul, Self);
        special_ops!($inner, saturating_pow, u32);
        special_ops!($inner, saturating_sub, Self);

        special_ops!($inner, wrapping_add, Self);
        special_ops!($inner, wrapping_div, Self);
        special_ops!($inner, wrapping_mul, Self);
        special_ops!($inner, wrapping_neg);
        special_ops!($inner, wrapping_pow, u32);
        special_ops!($inner, wrapping_rem, Self);
        special_ops!($inner, wrapping_shl, u32);
        special_ops!($inner, wrapping_shr, u32);
        special_ops!($inner, wrapping_sub, Self);
    }
}

macro_rules! unorm {
    ($struct:ident, $inner:ident) => {
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        #[repr(transparent)]
        pub struct $struct($inner);

        impl From<$inner> for $struct {
            fn from(n: $inner) -> Self {
                Self::from_inner(n)
            }
        }

        impl Into<$inner> for $struct {
            fn into(self) -> $inner {
                self.to_inner()
            }
        }

        impl std::convert::TryFrom<f32> for $struct {
            type Error = FloatToUnormError<f32>;

            fn try_from(f: f32) -> Result<Self, Self::Error> {
                Self::try_from_float(f)
            }
        }

        impl std::convert::TryFrom<f64> for $struct {
            type Error = FloatToUnormError<f64>;

            fn try_from(f: f64) -> Result<Self, Self::Error> {
                Self::try_from_float(f)
            }
        }

        impl std::fmt::Display for $struct {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.to_float::<f64>())
            }
        }

        ops!($struct, Self);

        ops!($struct, &Self);

        impl en::num_traits::Zero for $struct {
            fn zero() -> Self {
                Self::zero()
            }

            fn is_zero(&self) -> bool {
                *self == Self::zero()
            }
        }

        impl en::num_traits::One for $struct {
            fn one() -> Self {
                Self::one()
            }

            fn is_one(&self) -> bool {
                *self == Self::one()
            }
        }

        impl en::num_traits::Num for $struct {
            type FromStrRadixErr = <$inner as en::num_traits::Num>::FromStrRadixErr;

            fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
                $inner::from_str_radix(s, radix).map(Self::from_inner)
            }
        }

        impl en::num_traits::ToPrimitive for $struct {
            fn to_i64(&self) -> Option<i64> {
                self.to_inner().to_i64()
            }

            fn to_u64(&self) -> Option<u64> {
                self.to_inner().to_u64()
            }
        }

        impl en::num_traits::NumCast for $struct {
            fn from<T: en::num_traits::ToPrimitive>(n: T) -> Option<Self> {
                <$inner as en::num_traits::NumCast>::from(n).map(Self::from_inner)
            }
        }

        impl en::num_traits::Unsigned for $struct {}

        impl en::num_traits::Bounded for $struct {
            fn min_value() -> Self {
                Self::min_value()
            }

            fn max_value() -> Self {
                Self::max_value()
            }
        }

        impl $struct {
            pub const fn from_inner(inner: $inner) -> Self {
                Self(inner)
            }

            pub const fn zero() -> Self {
                Self::min_value()
            }

            pub const fn one() -> Self {
                Self::max_value()
            }

            pub const fn min_value() -> Self {
                Self::from_inner($inner::min_value())
            }

            pub const fn max_value() -> Self {
                Self::from_inner($inner::max_value())
            }

            pub const fn epsilon() -> Self {
                Self::from_inner(1)
            }

            pub fn from_denominator(denominator: $inner) -> Self {
                Self::from_inner(denominator_to_unorm(denominator))
            }

            pub fn try_from_float<F: en::Float>(f: F) -> Result<Self, FloatToUnormError<F>> {
                float_to_unorm(f).map(Self::from_inner)
            }

            pub fn from_float<F: en::Float>(f: F) -> Self {
                Self::try_from_float(f).unwrap()
            }

            pub fn from_float_clamped<F: en::Float>(f: F) -> Self {
                Self::from_inner(float_to_unorm_clamped(f))
            }

            pub const fn to_inner(self) -> $inner {
                self.0
            }

            pub fn to_float<F: en::Float>(self) -> F {
                unorm_to_float(self.to_inner())
            }

            fn map(self, f: impl FnOnce($inner) -> $inner) -> Self {
                Self::from_inner(f(self.to_inner()))
            }

            special_ops!($inner);
        }
    };

    ($bits:expr) => {
        paste::paste! {
            unorm!([<Unorm $bits:camel>], [<u $bits>]);
        }
    };
}
