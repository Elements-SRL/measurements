use crate::{
    measurement::Measurement, percentage, percentage::Percentage, prefix::Prefix, uom::Uom,
};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/// Represents a measurement range with a minimum, maximum, step size, and unit prefix.
///
/// # Type Parameters
/// - `U`: The unit of measurement, implementing the [`Uom`] trait.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct RangedMeasurement<U: Uom> {
    min: f64,
    max: f64,
    step: Option<f64>,
    prefix: Prefix,
    #[serde(skip)]
    uom: PhantomData<U>,
}

impl<U: Uom> RangedMeasurement<U> {
    /// Creates a new `RangedMeasurement` with the given minimum, maximum, step, and prefix.
    ///
    /// # Arguments
    /// * `min` - The minimum value of the range.
    /// * `max` - The maximum value of the range.
    /// * `step` - The step size between values in the range.
    /// * `prefix` - The SI prefix for the unit.
    pub fn new<V: Into<f64>>(min: V, max: V, step: V, prefix: Prefix) -> Self {
        Self {
            min: min.into(),
            max: max.into(),
            step: Some(step.into()),
            prefix,
            uom: PhantomData,
        }
    }
    /// Creates a new symmetrical `RangedMeasurement` with the given value, step, and prefix.
    ///
    /// # Arguments
    /// * `-value` - The minimum value of the range.
    /// * `value` - The maximum value of the range.
    /// * `step` - The step size between values in the range.
    /// * `prefix` - The SI prefix for the unit.
    pub fn new_sym<V: Into<f64>>(v: V, step: V, prefix: Prefix) -> Self {
        let v: f64 = v.into();
        Self {
            min: -v,
            max: v,
            step: Some(step.into()),
            prefix,
            uom: PhantomData,
        }
    }

    /// Creates a new symmetrical `RangedMeasurement` without any step with the given value, and prefix.
    ///
    /// # Arguments
    /// * `-value` - The minimum value of the range.
    /// * `value` - The maximum value of the range.
    /// * `prefix` - The SI prefix for the unit.
    pub fn new_sym_stepless<V: Into<f64>>(v: V, prefix: Prefix) -> Self {
        let v: f64 = v.into();
        Self {
            min: -v,
            max: v,
            step: None,
            prefix,
            uom: PhantomData,
        }
    }

    /// Returns the minimum value as a [`Measurement`] with the associated prefix.
    pub fn min(&self) -> Measurement<U> {
        Measurement::new(self.min, self.prefix)
    }

    /// Returns the maximum value as a [`Measurement`] with the associated prefix.
    pub fn max(&self) -> Measurement<U> {
        Measurement::new(self.max, self.prefix)
    }

    /// Returns the step size (if any) as a [`Option<Measurement>`] with the associated prefix.
    pub fn step(&self) -> Option<Measurement<U>> {
        self.step.map(|s| Measurement::new(s, self.prefix))
    }

    /// Checks if a given [`Measurement`] is within the range, optionally scaled by a [`Percentage`].
    ///
    /// # Arguments
    /// * `other` - The measurement to check.
    /// * `p` - An optional percentage to scale the range.
    ///
    /// # Returns
    /// `true` if `other` is within the scaled range, `false` otherwise.
    pub fn is_in_range(&self, other: Measurement<U>, p: Option<Percentage>) -> bool {
        let p = p.unwrap_or(percentage!(1.0)).get_value();
        other > self.min() * p && other < self.max() * p
    }

    /// Returns a string label combining min, max, step, prefix, and unit (e.g., "[-10.0,10.0,1.0]mV").
    pub fn label(&self) -> String {
        "[".to_string()
            + &self.min.to_string()
            + ","
            + &self.max.to_string()
            + ","
            + &self.step.map_or("".to_string(), |s| s.to_string())
            + "]"
            + self.prefix.get_label()
            + &U::uom()
    }

    /// Converts the ranged measurement to a different SI prefix, scaling the value accordingly.
    ///
    /// # Arguments
    /// * `pfx` - The target SI prefix.
    ///
    /// # Returns
    /// A new [`RangedMeasurement`] with the value converted to the target prefix.
    pub fn convert_to(&self, pfx: Prefix) -> Self {
        let cf = self.prefix.get_conversion_factor(pfx);
        Self {
            min: self.min * cf,
            max: self.max * cf,
            step: self.step.map(|s| s * cf) ,
            prefix: pfx,
            uom: PhantomData,
        }
    }
}

impl<U: Uom> PartialEq for RangedMeasurement<U> {
    fn eq(&self, other: &Self) -> bool {
        let t = if self.prefix == other.prefix {
            (self.min, self.max, self.step)
        } else {
            let new_rm = self.convert_to(other.prefix);
            (new_rm.min, new_rm.max, new_rm.step)
        };
        t == (other.min, other.max, other.step)
    }
}

impl<U: Uom> From<Measurement<U>> for RangedMeasurement<U> {
    fn from(value: Measurement<U>) -> Self {
        Self::new_sym_stepless(value.value(), value.prefix())
    }
}
#[cfg(test)]
mod ranged_measurement {
    use crate::uom::Volt;

    use super::*;

    #[test]
    fn get_min() {
        let r = RangedMeasurement::<Volt>::new(-10, 10, 1, Prefix::Micro);
        assert_eq!(r.min(), Measurement::new(-10, Prefix::Micro));
    }

    #[test]
    fn get_max() {
        let r = RangedMeasurement::<Volt>::new(-10, 10, 1, Prefix::Micro);
        assert_eq!(r.max(), Measurement::new(10, Prefix::Micro));
    }

    #[test]
    fn get_step() {
        let r = RangedMeasurement::<Volt>::new(-10, 10, 1, Prefix::Micro);
        assert_eq!(r.step(), Some(Measurement::new(1, Prefix::Micro)));
    }

    #[test]
    fn is_in_range_with_none() {
        let r = RangedMeasurement::<Volt>::new(-10, 10, 1, Prefix::Micro);
        assert!(r.is_in_range(Measurement::new(1, Prefix::Micro), None));
    }

    #[test]
    fn is_not_in_range_with_none() {
        let r = RangedMeasurement::<Volt>::new(-10, 10, 1, Prefix::Micro);
        assert!(!r.is_in_range(Measurement::new(1, Prefix::Kilo), None));
    }
    #[test]
    fn is_in_range_with_some() {
        let r = RangedMeasurement::<Volt>::new(-10, 10, 1, Prefix::Micro);
        assert!(r.is_in_range(Measurement::new(1, Prefix::Micro), Some(percentage!(0.5))));
    }

    #[test]
    fn is_not_in_range_with_some() {
        let r = RangedMeasurement::<Volt>::new(-10, 10, 1, Prefix::Micro);
        assert!(!r.is_in_range(Measurement::new(1, Prefix::Kilo), Some(percentage!(0.5))));
    }

    #[test]
    fn label() {
        let r = RangedMeasurement::<Volt>::new_sym(10, 1, Prefix::Micro);
        assert_eq!(r.label(), "[-10,10,1]uV");
    }

    #[test]
    fn equality_check() {
        let r = RangedMeasurement::<Volt>::new_sym(100, 1, Prefix::Micro);
        let r2 = RangedMeasurement::<Volt>::new_sym(0.1, 0.001, Prefix::Milli);
        assert_eq!(r, r2);
    }

    #[test]
    fn from_measurement() {
        let r = RangedMeasurement::<Volt>::new_sym_stepless(100, Prefix::Micro);
        assert_eq!(r, Measurement::new(100,  Prefix::Micro).into());
    }
}
