use num::traits::{Num, NumCast};

use super::mm::{MM, ToMM};
use super::m::{M, ToM};
use super::km::{KM, ToKM};

/// ToCM is the canonical trait to use for input in centimeters.
///
/// For example the millimeters type (MM) implements the ToCM trait and thus
/// millimeters can be given as a parameter to any input that seeks centimeters.
pub trait ToCM{
    type Output;

    /// to_cm returns these units in centimeters, performing conversion if needed.
    fn to_cm(self) -> CM<Self::Output>;
}

/// CM represents centimeters (1/100th a meter).
///
/// # Examples
///
/// ```
/// use fiz_math::unit::CM;
///
/// let x = CM(1.0);
/// println!("{:?}", x);
/// ```
unit!(CM);

impl<T: Num + NumCast> ToMM for CM<T> {
    type Output = T;

    /// to_mm returns these centimeters converted to millimeters.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::unit::{CM, MM, ToMM};
    ///
    /// assert_eq!(CM(1.0).to_mm(), MM(10.0));
    /// ```
    fn to_mm(self) -> MM<T> {
        MM(self.0 * T::from(10).unwrap())
    }
}

impl<T: Num + NumCast> ToCM for CM<T> {
    type Output = T;

    /// to_cm simply returns self.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::unit::{CM, ToCM};
    ///
    /// assert_eq!(CM(1.0).to_cm(), CM(1.0));
    /// ```
    fn to_cm(self) -> CM<T> {
        self
    }
}

impl<T: Num + NumCast> ToM for CM<T> {
    type Output = T;

    /// to_m returns these centimeters converted to meters.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::unit::{CM, M, ToM};
    ///
    /// assert_eq!(CM(100.0).to_m(), M(1.0));
    /// ```
    fn to_m(self) -> M<T> {
        M(self.0 / T::from(100).unwrap())
    }
}

impl<T: Num + NumCast> ToKM for CM<T> {
    type Output = T;

    /// to_km returns these centimeters converted to kilometers.
    ///
    /// # Examples
    ///
    /// ```
    /// use fiz_math::unit::{CM, KM, ToKM};
    ///
    /// assert_eq!(CM(100000.0).to_km(), KM(1.0));
    /// ```
    fn to_km(self) -> KM<T> {
        KM(self.0 / T::from(100000).unwrap())
    }
}
