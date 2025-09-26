use crate::{m1d::M1d, prefix::Prefix, prelude::Measurement, uom::Uom};
use ndarray::{concatenate, Array2, Axis, Dimension, SliceArg};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/// A two-dimensional array of measurements with a unit and SI prefix.
///
/// # Type Parameters
/// - `U`: The unit of measurement, implementing the [`Uom`] trait.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct M2d<U: Uom> {
    values: Array2<f64>,
    prefix: Prefix,
    #[serde(skip)]
    uom: PhantomData<U>,
}

impl<U: Uom> M2d<U> {
    /// Creates a new [`M2d`] with the given values and prefix.
    ///
    /// # Arguments
    /// * `values` - The values as a type convertible into `Array2<f64>`.
    /// * `prefix` - The SI prefix for the unit.
    /// # Example
    /// ```
    /// use typed_measurements::prelude::*;
    /// use ndarray::Array2;
    ///
    /// let arr = Array2::from_shape_vec((2, 2), vec![1.0, 2.0, 3.0, 4.0]).unwrap();
    /// let m = M2d::<Volt>::new(arr, Prefix::Milli);
    /// ```
    pub fn new<T: Into<Array2<f64>>>(values: T, prefix: Prefix) -> Self {
        Self {
            values: values.into(),
            prefix,
            uom: PhantomData,
        }
    }

    /// Returns a clone of the underlying values array.
    pub fn values(&self) -> Array2<f64> {
        self.values.clone()
    }

    /// Returns the SI prefix associated with this array.
    pub fn prefix(&self) -> Prefix {
        self.prefix
    }

    /// Returns the mean value of all elements as a [`Measurement<U>`].
    ///
    /// # Returns
    /// An `Option<Measurement<U>>` containing the mean, or `None` if the array is empty.
    pub fn mean(&self) -> Option<Measurement<U>> {
        Some(Measurement::new(self.values.mean()?, self.prefix()))
    }

    /// Returns the mean along the specified axis as an [`M1d<U>`].
    ///
    /// # Arguments
    /// * `axis` - The axis along which to compute the mean.
    ///
    /// # Returns
    /// An `Option<M1d<U>>` containing the mean values, or `None` if the axis is invalid.
    pub fn mean_axis(&self, axis: Axis) -> Option<M1d<U>> {
        Some(M1d::new(self.values.mean_axis(axis)?, self.prefix()))
    }

    /// Returns the std dev along the specified axis as an [`M1d<U>`].
    ///
    /// # Arguments
    /// * `axis` - The axis along which to compute the std dev.
    ///
    /// # Returns
    /// An `M1d<U>` containing the std values.
    pub fn std_axis(&self, axis: Axis, ddof: f64) -> M1d<U> {
        M1d::new(self.values.std_axis(axis, ddof), self.prefix())
    }
    /// Returns a clone of the underlying values array.
    ///
    /// # Returns
    /// A copy of the internal `Array2<f64>`.
    pub fn label(&self) -> String {
        self.mean()
            .map_or(Measurement::new(0, self.prefix()), |f| f)
            .label()
    }

    /// Converts the array to a different SI prefix, scaling all values accordingly.
    ///
    /// # Arguments
    /// * `pfx` - The target SI prefix.
    ///
    /// # Returns
    /// A new [`M2d`] with values converted to the target prefix.
    pub fn convert_to(self, pfx: Prefix) -> Self {
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

    /// Returns the length of the inside 2d array.
    ///
    /// # Returns
    /// The length of the inside 2d array as usize.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn concatenate_axis(&self, other: &M2d<U>, axis: Axis) -> M2d<U> {
        let other = if self.prefix != other.prefix {
            other.clone().convert_to(self.prefix())
        } else {
            other.clone()
        };
        M2d::new(concatenate![axis, self.values(), other.values()], self.prefix())
    }
}

impl<U: Uom> PartialEq for M2d<U> {
    /// Compares two [`M2d`] arrays for equality, converting prefixes if necessary.
    fn eq(&self, other: &Self) -> bool {
        if self.prefix != other.prefix {
            self.clone().convert_to(other.prefix()).values == other.values
        } else {
            self.values == other.values
        }
    }
}

#[cfg(test)]
mod m2d_tests {
    use super::*;
    use crate::uom::Volt;

    #[test]
    fn get_values() {
        let m = M2d::<Volt>::new(
            Array2::from_shape_vec((2, 2), vec![1000.0, 2000.0, 3000.0, 4000.0]).unwrap(),
            Prefix::Milli,
        );
        assert_eq!(
            m.values(),
            Array2::from_shape_vec((2, 2), vec![1000.0, 2000.0, 3000.0, 4000.0]).unwrap()
        );
    }

    #[test]
    fn convert_to() {
        let m = M2d::<Volt>::new(
            Array2::from_shape_vec((2, 2), vec![1.0, 2.0, 3.0, 4.0]).unwrap(),
            Prefix::Milli,
        );
        assert_eq!(
            m.convert_to(Prefix::Micro).values(),
            Array2::from_shape_vec((2, 2), vec![1000.0, 2000.0, 3000.0, 4000.0]).unwrap()
        );
    }

    #[test]
    fn convert_to_2() {
        let m1 = M2d::<Volt>::new(
            Array2::from_shape_vec((2, 2), vec![1.0, 2.0, 3.0, 4.0]).unwrap(),
            Prefix::Milli,
        );
        let m2 = M2d::<Volt>::new(
            Array2::from_shape_vec((2, 2), vec![1000.0, 2000.0, 3000.0, 4000.0]).unwrap(),
            Prefix::Micro,
        );
        assert_eq!(m1, m2);
    }

    #[test]
    fn convert_to_3() {
        // should short-circuit if prefixes are the same
        let m = M2d::<Volt>::new(
            Array2::from_shape_vec((2, 2), vec![1.0, 2.0, 3.0, 4.0]).unwrap(),
            Prefix::Milli,
        );
        let m2 = m.clone();
        assert_eq!(m, m2);
    }
}
