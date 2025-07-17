use crate::{
    measurement::Measurement, percentage, percentage::Percentage, prefix::Prefix, uom::Uom,
};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct RangedMeasurement<U: Uom> {
    min: f64,
    max: f64,
    step: f64,
    prefix: Prefix,
    uom: PhantomData<U>,
}

impl<U: Uom> RangedMeasurement<U> {
    pub fn new<V: Into<f64>>(min: V, max: V, step: V, prefix: Prefix) -> Self {
        Self {
            min: min.into(),
            max: max.into(),
            step: step.into(),
            prefix,
            uom: PhantomData,
        }
    }

    fn min(&self) -> Measurement<U> {
        Measurement::new(self.min, self.prefix)
    }

    fn max(&self) -> Measurement<U> {
        Measurement::new(self.max, self.prefix)
    }

    fn step(&self) -> Measurement<U> {
        Measurement::new(self.step, self.prefix)
    }

    fn is_in_range(&self, other: Measurement<U>, p: Option<Percentage>) -> bool {
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
