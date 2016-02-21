pub trait Clamp{
    type Elem;

    /// clamp returns the value v clamped to the range of [min, max].
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::Clamp;
    ///
    /// assert_eq!(1.1.clamp(0.0, 1.0), 1.0);
    /// assert_eq!(2.0.clamp(0.0, 1.5), 1.5);
    ///
    /// assert_eq!((-1.0).clamp(0.0, 1.0), 0.0);
    /// assert_eq!(0.0.clamp(0.1, 1.0), 0.1);
    /// ```
    ///
    /// ```
    /// use fiz_math::Clamp;
    ///
    /// assert_eq!(11.clamp(0, 10), 10);
    /// assert_eq!(20.clamp(0, 15), 15);
    ///
    /// assert_eq!((-10).clamp(0, 10), 0);
    /// assert_eq!(0.clamp(1, 10), 1);
    /// ```
    fn clamp(self, min: Self::Elem, max: Self::Elem) -> Self;
}

macro_rules! impl_clamp_floats {
    ($($ty:ty),*) => ($(
        impl Clamp for $ty {
            type Elem = $ty;

            fn clamp(self, min: $ty, max: $ty) -> Self {
                self.min(max).max(min)
            }
        }
    )*);
}

macro_rules! impl_clamp_ints {
    ($($ty:ty),*) => ($(
        impl Clamp for $ty {
            type Elem = $ty;

            fn clamp(self, min: $ty, max: $ty) -> Self {
                if self < min {
                    min
                } else if self > max {
                    max
                } else {
                    self
                }
            }
        }
    )*);
}

impl_clamp_floats! { f32, f64 }
impl_clamp_ints! { i8, u8, i16, u16, i32, u32, i64, u64, isize, usize }
