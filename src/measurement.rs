use crate::{prefix::Prefix, uom::Uom};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    marker::PhantomData,
    ops::{Add, Sub},
};

/// Represents a physical measurement with a value, SI prefix, and unit.
///
/// # Type Parameters
/// - `U`: The unit of measurement, implementing the [`Uom`] trait.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Measurement<U: Uom> {
    value: f64,
    prefix: Prefix,
    #[serde(skip)]
    uom: PhantomData<U>,
}

impl<U: Uom> Measurement<U> {
    /// Creates a new [`Measurement`] with the given value and prefix.
    ///
    /// # Arguments
    /// * `value` - The numeric value of the measurement.
    /// * `prefix` - The SI prefix for the unit.
    pub fn new<V: Into<f64>>(value: V, prefix: Prefix) -> Self {
        Self {
            value: value.into(),
            prefix,
            uom: PhantomData,
        }
    }

    /// Returns the numeric value of the measurement.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Returns a string label combining value, prefix, and unit (e.g., "1.0mV").
    pub fn label(&self) -> String {
        self.value.to_string() + self.prefix.get_label() + &U::uom()
    }

    /// Converts the measurement to a different SI prefix, scaling the value accordingly.
    ///
    /// # Arguments
    /// * `pfx` - The target SI prefix.
    ///
    /// # Returns
    /// A new [`Measurement`] with the value converted to the target prefix.
    pub fn convert_to(&self, pfx: Prefix) -> Self {
        Measurement {
            value: self.value * self.prefix.get_conversion_factor(pfx),
            prefix: pfx,
            uom: PhantomData,
        }
    }

    /// Returns the SI prefix associated with this measurement.
    pub fn prefix(&self) -> Prefix {
        self.prefix
    }

    /// Returns a "nice" representation of the measurement, adjusting the prefix for readability.
    pub fn nice(self) -> Self {
        let original_prefix = self.prefix();
        let (e, s) = if self.value > 1.0 {
            (self.value, 1)
        } else {
            (1.0 / self.value, -1)
        };
        let exp = e.log10() as i16;
        if exp < 3 {
            return self;
        }
        let (p, _) = Prefix::from_exp_value(exp * s);
        self.convert_to(p * original_prefix)
    }
}

impl<U: Uom> Add for Measurement<U> {
    /// Adds two [`Measurement`]s, converting to the same prefix if necessary.
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let pfx = rhs.prefix;
        let s = self.convert_to(pfx);
        Self {
            value: s.value + rhs.value,
            prefix: pfx,
            uom: PhantomData,
        }
    }
}

impl<U: Uom> Sub for Measurement<U> {
    /// Subtracts two [`Measurement`]s, converting to the same prefix if necessary.
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let pfx = rhs.prefix;
        let s = self.convert_to(pfx);
        Self {
            value: s.value - rhs.value,
            prefix: pfx,
            uom: PhantomData,
        }
    }
}

impl<U: Uom> PartialEq for Measurement<U> {
    /// Checks equality between two [`Measurement`]s, converting to the same prefix if necessary.
    fn eq(&self, other: &Self) -> bool {
        self.convert_to(other.prefix()).value == other.value
    }
}

impl<U: Uom> PartialOrd for Measurement<U> {
    /// Compares two [`Measurement`]s, converting to the same prefix if necessary.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let m1 = self.convert_to(other.prefix()).value();
        let m2 = other.value();
        if m1.is_nan() || m2.is_nan() {
            None
        } else if m1 == m2 {
            Some(Ordering::Equal)
        } else if m1 < m2 {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl<U: Uom, I: Into<f64>> std::ops::Mul<I> for Measurement<U> {
    /// Multiplies a [`Measurement`] by a scalar.
    type Output = Measurement<U>;
    fn mul(self, rhs: I) -> Self::Output {
        Measurement::new(self.value * rhs.into(), self.prefix)
    }
}

impl<U: Uom, I: Into<f64>> std::ops::Div<I> for Measurement<U> {
    /// Divides a [`Measurement`] by a scalar.
    type Output = Measurement<U>;
    fn div(self, rhs: I) -> Self::Output {
        Measurement::new(self.value / rhs.into(), self.prefix)
    }
}

#[cfg(test)]
mod measurement_tests {
    use super::*;
    use crate::uom::Volt;

    #[test]
    fn kilo_plus_kilo() {
        let a = Measurement::<Volt>::new(1, Prefix::Kilo);
        let b = Measurement::<Volt>::new(41, Prefix::Kilo);
        assert_eq!(a + b, Measurement::<Volt>::new(42, Prefix::Kilo));
    }

    #[test]
    fn kilo_minus_kilo() {
        let a = Measurement::<Volt>::new(1, Prefix::Kilo);
        let b = Measurement::<Volt>::new(43, Prefix::Kilo);
        assert_eq!(b - a, Measurement::<Volt>::new(42, Prefix::Kilo));
    }

    #[test]
    fn useless_nice() {
        let a = Measurement::<Volt>::new(1, Prefix::Kilo);
        assert_eq!(a, a.nice());
    }

    #[test]
    fn useless_nice_2() {
        let a = Measurement::<Volt>::new(10, Prefix::Kilo);
        assert_eq!(a, a.nice());
    }

    #[test]
    fn useless_nice_3() {
        let a = Measurement::<Volt>::new(100, Prefix::Kilo);
        assert_eq!(a.value, a.nice().value);
    }

    #[test]
    fn useful_nice() {
        let a = Measurement::<Volt>::new(1000, Prefix::Kilo);
        let nice = a.nice();
        assert_eq!(nice.value, 1.0);
        assert_eq!(Prefix::Mega, nice.prefix);
    }

    #[test]
    fn useful_nice_2() {
        let a = Measurement::<Volt>::new(10000, Prefix::Kilo);
        let nice = a.nice();
        assert_eq!(nice.value, 10.0);
        assert_eq!(Prefix::Mega, nice.prefix);
    }

    #[test]
    fn useful_nice_3() {
        let a = Measurement::<Volt>::new(0.001, Prefix::Kilo);
        let nice = a.nice();
        assert_eq!(nice.value, 1.0);
        assert_eq!(Prefix::None, nice.prefix);
    }

    #[test]
    fn equality_check() {
        let a = Measurement::<Volt>::new(0.001, Prefix::Kilo);
        assert_eq!(a, a.nice());
    }

    #[test]
    fn label_correctness() {
        let a = Measurement::<Volt>::new(0.125, Prefix::Milli);
        assert_eq!(a.label(), "0.125mV");
    }

    #[test]
    fn value_correctness() {
        let a = Measurement::<Volt>::new(0.125, Prefix::Milli);
        assert_eq!(a.value(), 0.125);
    }

    #[test]
    fn sum() {
        let a = Measurement::<Volt>::new(1, Prefix::Milli);
        let b = Measurement::new(125, Prefix::Micro);
        assert_eq!(a + b, Measurement::<Volt>::new(1.125, Prefix::Milli));
    }

    #[test]
    fn diff() {
        let a = Measurement::<Volt>::new(1, Prefix::Milli);
        let b = Measurement::new(1, Prefix::Milli);
        assert_eq!(a - b, Measurement::<Volt>::new(0, Prefix::Milli));
    }

    #[test]
    fn ord_0() {
        let a = Measurement::<Volt>::new(1, Prefix::Milli);
        let b = Measurement::new(1, Prefix::Micro);
        assert!(a > b);
    }

    #[test]
    fn ord_1() {
        let a = Measurement::<Volt>::new(1, Prefix::Milli);
        let b = Measurement::new(1, Prefix::Micro);
        assert!(a > b);
    }

    #[test]
    fn ord_2() {
        let a = Measurement::<Volt>::new(1, Prefix::Milli);
        let b = Measurement::new(1, Prefix::Micro);
        assert!(b < a);
    }
    #[test]
    fn ord_3() {
        let a = Measurement::<Volt>::new(1, Prefix::Milli);
        assert_eq!(a, a);
    }
}
