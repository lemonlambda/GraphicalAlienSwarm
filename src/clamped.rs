use paste::paste;

/// Clamps values to certain range and keeps it there
/// WARNING: This has runtime overhead
pub struct Clamp<T> {
    min: T,
    max: T,
    pub value: T,
}

impl<T> Clamp<T> {
    pub fn new(min: T, max: T, value: T) -> Self {
        Self { min, max, value }
    }
}

macro_rules! impl_has_min_max {
    ($($t:ty),*) => {
        $(
            impl HasMinMax<$t> for $t {
                const MIN: $t = <$t>::MIN;
                const MAX: $t = <$t>::MAX;
            }
        )*
    };
}

trait HasMinMax<T> {
    const MIN: T;
    const MAX: T;
}
impl_has_min_max! {
    u8, u16, u32, u64, u128,
    i8, i16, i32, i64, i128,
    f32, f64
}

impl<T: HasMinMax<T>> From<T> for Clamp<T> {
    fn from(value: T) -> Self {
        Self {
            min: T::MIN,
            max: T::MAX,
            value,
        }
    }
}

macro_rules! clamp_ops {
    ($($lit:ident, $lower_case:ident),*) => {
        $(
            paste! {
                impl<T:
                    Clone +
                    std::cmp::PartialOrd +
                    std::cmp::PartialEq +
                    std::ops::$lit<Output = T> +
                    num_traits::ops::saturating::[<Saturating $lit>]
                > std::ops::$lit<T> for Clamp<T> {
                    type Output = Clamp<T>;

                    fn [<$lit:lower>](mut self, rhs: T) -> Self::Output {
                            let new_value = self.value.[<saturating_ $lit:lower>](&rhs);

                        if self.min >= new_value {
                            self.value = self.min.clone();
                        } else if self.max <= new_value {
                            self.value = self.max.clone();
                        } else {
                            self.value = new_value;
                        }

                        return self;
                    }
                }
            }
        )*
    }
}

clamp_ops! {
    Add,
    Mul,
    Sub,
    Div
}
