use num::traits::{Num, NumCast};

use super::cm::{CM, ToCM};
use super::mm::{MM, ToMM};
use super::m::{M, ToM};

/// ToKM is the canonical trait to use for taking input in kilometers.
///
/// For example the millimeters type (MM) implements the ToKM trait and thus
/// millimeters can be given as a parameter to any input that seeks kilometers.
pub trait ToKM{
    type Output;

    /// to_km returns these units in kilometers, performing conversion if needed.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::unit::{M, ToKM};
    /// use fiz_math::Num;
    /// use std::fmt::Debug;
    ///
    /// fn walk<T: ToKM<Output=U>, U: Num+Debug>(dist: T) {
    ///     println!("{:?}", dist.to_km().0)
    /// }
    /// walk(M(2.0));
    /// walk(M::<i32>(2));
    /// ```
    fn to_km(self) -> KM<Self::Output>;
}

/// KM represents kilometers (1000 meters).
///
/// # Examples
///
/// ```
/// use fiz_math::unit::KM;
///
/// let x = KM(1.0);
/// println!("{:?}", x);
/// ```
unit!(KM);

impl<T: Num + NumCast> ToMM for KM<T> {
    type Output = T;

    /// to_mm returns these kilometers converted to millimeters.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::unit::{KM, MM, ToMM};
    ///
    /// assert_eq!(KM(1.0).to_mm(), MM(1000000.0));
    /// ```
    fn to_mm(self) -> MM<T> {
        MM(self.0 * T::from(1000000).unwrap())
    }
}

impl<T: Num + NumCast> ToCM for KM<T> {
    type Output = T;

    /// to_cm returns these kilometers converted to centimeters.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::unit::{KM, CM, ToCM};
    ///
    /// assert_eq!(KM(1.0).to_cm(), CM(100000.0));
    /// ```
    fn to_cm(self) -> CM<T> {
        CM(self.0 * T::from(100000).unwrap())
    }
}

impl<T: Num + NumCast> ToM for KM<T> {
    type Output = T;

    /// to_m returns these kilometers converted to meters.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::unit::{KM, M, ToM};
    ///
    /// assert_eq!(KM(1.0).to_m(), M(1000.0));
    /// ```
    fn to_m(self) -> M<T> {
        M(self.0 * T::from(1000).unwrap())
    }
}

impl<T: Num + NumCast> ToKM for KM<T> {
    type Output = T;

    /// to_km simply returns self.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::unit::{KM, ToKM};
    ///
    /// assert_eq!(KM(1.0).to_km(), KM(1.0));
    /// ```
    fn to_km(self) -> KM<T> {
        self
    }
}
