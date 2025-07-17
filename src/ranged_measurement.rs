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
    step: f64,
    prefix: Prefix,
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
            step: step.into(),
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

    /// Returns the step size as a [`Measurement`] with the associated prefix.
    pub fn step(&self) -> Measurement<U> {
        Measurement::new(self.step, self.prefix)
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
        assert_eq!(r.step(), Measurement::new(1, Prefix::Micro));
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
}
