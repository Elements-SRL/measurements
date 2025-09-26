use crate::{prefix::Prefix, prelude::Measurement, uom::Uom};
use ndarray::Array1;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/// A one-dimensional array of measurements with a unit and SI prefix.
///
/// # Type Parameters
/// - `U`: The unit of measurement, implementing the [`Uom`] trait.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct M1d<U: Uom> {
    values: Array1<f64>,
    prefix: Prefix,
    #[serde(skip)]
    uom: PhantomData<U>,
}

impl<U: Uom> M1d<U> {
    /// Creates a new [`M1d`] with the given values and prefix.
    ///
    /// # Arguments
    /// * `values` - The values as a type convertible into `Array1<f64>`.
    /// * `prefix` - The SI prefix for the unit.
    pub fn new<T: Into<Array1<f64>>>(values: T, prefix: Prefix) -> Self {
        Self {
            values: values.into(),
            prefix,
            uom: PhantomData,
        }
    }

    /// Returns a clone of the underlying values array.
    pub fn values(&self) -> Array1<f64> {
        self.values.clone()
    }

    /// Returns the SI prefix associated with this array.
    pub fn prefix(&self) -> Prefix {
        self.prefix
    }

    /// Converts the array to a different SI prefix, scaling all values accordingly.
    ///
    /// # Arguments
    /// * `pfx` - The target SI prefix.
    ///
    /// # Returns
    /// A new [`M1d`] with values converted to the target prefix.
    fn convert_to(self, pfx: Prefix) -> Self {
        let conversion_factor = self.prefix.get_conversion_factor(pfx);
        if conversion_factor == 1.0 {
            self.clone()
        } else {
            let mut s = self;
            s.values.par_mapv_inplace(|x| x * conversion_factor);
            Self {
                values: s.values,
                prefix: pfx,
                uom: PhantomData,
            }
        }
    }

    /// Returns the mean value of all elements as a [`Measurement<U>`].
    ///
    /// # Returns
    /// An `Option<Measurement<U>>` containing the mean, or `None` if the array is empty.
    pub fn mean(&self) -> Option<Measurement<U>> {
        Some(Measurement::new(self.values.mean()?, self.prefix))
    }
    
    /// Returns the len of the embedded array.
    ///
    /// # Returns
    /// An `usize` as the len.
    pub fn len(&self) -> usize {
        self.values.len()
    }
    
    /// Return whether the array has any elements
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl<U: Uom> PartialEq for M1d<U> {
    /// Compares two [`M1d`] arrays for equality, converting prefixes if necessary.
    fn eq(&self, other: &Self) -> bool {
        if self.prefix != other.prefix {
            self.clone().convert_to(other.prefix()).values == other.values
        } else {
            self.values == other.values
        }
    }
}

#[cfg(test)]
mod m1d_tests {
    use super::*;
    use crate::uom::Volt;

    #[test]
    fn get_values() {
        let m1d = M1d::<Volt>::new(vec![1.0, 2.0, 3.0], Prefix::Milli);
        assert_eq!(m1d.values(), Array1::from(vec![1.0, 2.0, 3.0]));
    }

    #[test]
    fn convert_to() {
        let m1d = M1d::<Volt>::new(vec![1.0, 2.0, 3.0], Prefix::Milli);
        assert_eq!(
            m1d.convert_to(Prefix::Micro).values(),
            Array1::from(vec![1000.0, 2000.0, 3000.0])
        );
    }

    #[test]
    fn convert_to_2() {
        let m1d1 = M1d::<Volt>::new(vec![1.0, 2.0, 3.0], Prefix::Milli);
        let m1d2 = M1d::<Volt>::new(vec![1000.0, 2000.0, 3000.0], Prefix::Micro);
        assert_eq!(m1d1, m1d2);
    }

    #[test]
    fn convert_to_3() {
        // should short-circuit if prefixes are the same
        let m1d1 = M1d::<Volt>::new(vec![1.0, 2.0, 3.0], Prefix::Milli);
        let m2 = m1d1.clone();
        assert_eq!(m1d1, m2);
    }
}
