use num::traits::{Num, NumCast};

use super::cm::{CM, ToCM};
use super::m::{M, ToM};
use super::km::{KM, ToKM};

/// ToMM is the canonical trait to use for input in millimeters.
///
/// For example the centimeters type (CM) implements the ToMM trait and
/// thus centimeters can be given as a parameter to any input that seeks
/// millimeters).
pub trait ToMM{
    type Output;

    /// to_mm returns these units in millimeters, performing conversion if
    /// needed.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::unit::{MM, ToMM};
    /// use fiz_math::Num;
    /// use std::fmt::Debug;
    ///
    /// fn walk<T: ToMM<Output=U>, U: Num+Debug>(dist: T) {
    ///     println!("{:?}", dist.to_mm().0)
    /// }
    /// walk(MM(2.0));
    /// walk(MM::<i8>(2));
    /// ```
    fn to_mm(self) -> MM<Self::Output>;
}

/// MM represents millimeters (1/10th a centimeter).
///
/// # Examples
///
/// ```
/// use fiz_math::unit::MM;
///
/// let x = MM(1.0);
/// println!("{:?}", x);
/// ```
unit!(MM);

impl<T: Num + NumCast> ToMM for MM<T> {
    type Output = T;

    /// to_mm simply returns self.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::unit::{MM, ToMM};
    ///
    /// assert_eq!(MM(1.0).to_mm(), MM(1.0));
    /// ```
    fn to_mm(self) -> MM<T> {
        self
    }
}

impl<T: Num + NumCast> ToCM for MM<T> {
    type Output = T;

    /// to_cm returns these millimeters converted to centimeters.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::unit::{MM, CM, ToCM};
    ///
    /// assert_eq!(MM(10.0).to_cm(), CM(1.0));
    /// ```
    fn to_cm(self) -> CM<T> {
        CM(self.0 / T::from(10).unwrap())
    }
}

impl<T: Num + NumCast> ToM for MM<T> {
    type Output = T;

    /// to_m returns these millimeters converted to meters.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::unit::{MM, M, ToM};
    ///
    /// assert_eq!(MM(1000.0).to_m(), M(1.0));
    /// ```
    fn to_m(self) -> M<T> {
        M(self.0 / T::from(1000).unwrap())
    }
}

impl<T: Num + NumCast> ToKM for MM<T> {
    type Output = T;

    /// to_km returns these millimeters converted to kilometers.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::unit::{MM, KM, ToKM};
    ///
    /// assert_eq!(MM(1000000.0).to_km(), KM(1.0));
    /// ```
    fn to_km(self) -> KM<T> {
        KM(self.0 / T::from(1000000).unwrap())
    }
}
